use core::fmt;
use std::{
    future::{ready, Ready},
    str::Utf8Error,
    sync::Arc,
};

use actix_http::body::{EitherBody, MessageBody};
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, ResponseError,
};
use futures_util::{future::LocalBoxFuture, StreamExt};
use parcel_common::api_types::EncryptedData;

#[derive(Debug, thiserror::Error)]
enum EncryptionError {
    InvalidUtf8Body(Utf8Error),
    InvalidJsonData(serde_json::Error),
    InvalidAesData,
}

impl fmt::Display for EncryptionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EncryptionError::InvalidUtf8Body(_) => {
                write!(f, "The content body contains invalid utf8 data")
            }
            EncryptionError::InvalidJsonData(_) => {
                write!(f, "The content body could not be deserialized")
            }
            EncryptionError::InvalidAesData => write!(f, "The content body could not be decrypted"),
        }
    }
}

impl ResponseError for EncryptionError {
    fn status_code(&self) -> actix_http::StatusCode {
        actix_http::StatusCode::BAD_REQUEST
    }
}

#[derive(Default)]
pub struct DataEncryption;

impl<S, B> Transform<S, ServiceRequest> for DataEncryption
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = DataEncryptionMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(DataEncryptionMiddleware {
            service: Arc::new(service),
        }))
    }
}

pub struct DataEncryptionMiddleware<S> {
    service: Arc<S>,
}

impl<S, B> Service<ServiceRequest> for DataEncryptionMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();

        Box::pin(async move {
            let use_encryption;
            let use_encryption_header = req.headers().get("Use-Encryption");

            if let Some(use_encryption_header) = use_encryption_header {
                use_encryption = use_encryption_header
                    .to_str()
                    .unwrap_or("true")
                    .parse::<bool>()
                    .unwrap_or(true);
            } else {
                use_encryption = true;
            }

            if use_encryption {
                let mut payload = req.take_payload();
                let mut req_body = Vec::with_capacity(1024);
                while let Some(chunk) = payload.next().await {
                    req_body.extend_from_slice(&chunk?);
                }

                if !req_body.is_empty() {
                    let body = core::str::from_utf8(&req_body).map(|body| body.trim());

                    match body {
                        Ok(body) => {
                            // todo: parse to EncryptedData struct
                            match serde_json::from_str::<EncryptedData>(body) {
                                Ok(encrypted_data) => {
                                    if let Some(data) = encrypted_data.data {
                                        // todo: decrypt string in EncryptedData::data

                                        // todo: set decrypted string as payload

                                        return Ok(req
                                            .error_response(EncryptionError::InvalidAesData)
                                            .map_into_right_body());
                                    }
                                }
                                Err(err) => {
                                    return Ok(req
                                        .error_response(EncryptionError::InvalidJsonData(err))
                                        .map_into_right_body())
                                }
                            }
                        }
                        Err(err) => {
                            return Ok(req
                                .error_response(EncryptionError::InvalidUtf8Body(err))
                                .map_into_right_body())
                        }
                    }
                }
            }

            let res = service.call(req).await?;

            if use_encryption {
                // todo: get body from response (if any)

                // todo: encrypt body

                // todo: create EncryptedData and serialize to json

                // todo: set json string as response payload
            }

            Ok(res.map_into_left_body())
        })
    }
}
