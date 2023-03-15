use std::{
    cell::RefCell,
    collections::BTreeMap,
    future::{ready, Ready},
    rc::Rc,
};

use actix_http::{
    body::{BoxBody, EitherBody, MessageBody},
    HttpMessage,
};
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web::{Buf, Bytes},
    Error,
};
use futures_util::{future::LocalBoxFuture, StreamExt};
use serde_json::Value;

#[derive(Default)]
pub struct DeepLogger {
    pub enabled: bool,
}

impl<S, B> Transform<S, ServiceRequest> for DeepLogger
where
    S: 'static + Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: MessageBody + From<BoxBody> + 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = DeepLoggerMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(DeepLoggerMiddleware {
            service: Rc::new(RefCell::new(service)),
            enabled: self.enabled,
        }))
    }
}

pub struct DeepLoggerMiddleware<S> {
    service: Rc<RefCell<S>>,
    enabled: bool,
}

impl<S, B> Service<ServiceRequest> for DeepLoggerMiddleware<S>
where
    S: 'static + Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: MessageBody + From<BoxBody> + 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        if !self.enabled {
            let fut = self.service.call(req);
            return Box::pin(async move { Ok(fut.await?.map_into_left_body()) });
        }

        let svc = self.service.clone();

        Box::pin(async move {
            let body = get_request_body(&mut req).await?;

            if !body.is_empty() {
                let json_data = serde_json::from_slice::<BTreeMap<String, Value>>(&body)?;
                let formatted_body = serde_json::to_string_pretty(&json_data)?;

                req.set_payload(create_payload(formatted_body.clone().into()).into());

                log::debug!("{} request body: {}", req.path(), formatted_body);
            } else {
                log::debug!("{} request body: (empty)", req.path());
            }

            let service_res = svc.call(req).await?;

            if service_res.response().error().is_none() {
                let (req, res) = service_res.into_parts();
                let (res, body) = res.into_parts();

                let body = get_response_body(body);

                if !body.is_empty() {
                    let json_data = serde_json::from_slice::<BTreeMap<String, Value>>(&body)?;
                    let formatted_body = serde_json::to_string_pretty(&json_data)?;

                    log::debug!("{} response body: {}", req.path(), formatted_body);
                } else {
                    log::debug!("{} response body: (empty)", req.path());
                }

                let res = res.set_body(B::from(BoxBody::new(body)));
                let service_res = ServiceResponse::new(req, res).map_into_left_body();

                Ok(service_res)
            } else {
                Ok(service_res.map_into_left_body())
            }
        })
    }
}

async fn get_request_body(req: &mut ServiceRequest) -> Result<Vec<u8>, Error> {
    let mut payload = req.take_payload();
    let mut body = Vec::new();

    while let Some(chunk) = payload.next().await {
        body.extend_from_slice(chunk?.chunk());
    }

    Ok(body)
}

fn create_payload(bytes: Bytes) -> actix_http::h1::Payload {
    let (_, mut payload) = actix_http::h1::Payload::create(true);
    payload.unread_data(bytes);
    payload
}

fn get_response_body(body: impl MessageBody) -> Bytes {
    let bytes = body.try_into_bytes();

    match bytes {
        Ok(bytes) => bytes,
        Err(_) => Bytes::new(),
    }
}
