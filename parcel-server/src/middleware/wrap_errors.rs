use std::future::{ready, Ready};

use actix_http::{
    body::{BoxBody, EitherBody, MessageBody},
    HttpMessage, StatusCode,
};
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web, Error,
};
use futures_util::future::LocalBoxFuture;

use crate::response_error::CommonError;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
#[derive(Default)]
pub struct WrapErrors;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for WrapErrors
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = WrapErrorsMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(WrapErrorsMiddleware { service }))
    }
}

pub struct WrapErrorsMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for WrapErrorsMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            if res.response().error().is_some() || res.response().status() != StatusCode::OK {
                let (req, res) = res.into_parts();
                let (res, body) = res.into_parts();
                let bytes = body.try_into_bytes();

                let new_response = match bytes {
                    Ok(bytes) => {
                        // try parse as CommonError, if it fails we should replace the original error as it might contain sensitive information
                        if serde_json::from_slice::<CommonError>(&bytes).is_err() {
                            log::debug!(
                                "Wrapping error response. Status = {}, Error = {:#?}",
                                res.status(),
                                res.error()
                            );

                            let res_error = CommonError {
                                message: "internal error".into(),
                                status: "SV-IE".into(),
                                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                            };

                            let bytes =
                                web::Bytes::from(serde_json::to_string(&res_error).unwrap());

                            res.set_body(BoxBody::new(bytes)).map_into_right_body()
                        } else {
                            res.set_body(BoxBody::new(bytes)).map_into_right_body()
                        }
                    }
                    Err(body) => res.set_body(body).map_into_left_body(),
                };

                Ok(ServiceResponse::new(req, new_response))
            } else {
                Ok(res.map_into_left_body())
            }
        })
    }
}
