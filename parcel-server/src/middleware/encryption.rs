use core::fmt;
use std::{
    future::{ready, Ready},
    str::Utf8Error,
    sync::Arc,
};

use actix_http::body::{BoxBody, EitherBody, MessageBody};
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web::{Bytes, BytesMut},
    Error, HttpMessage, ResponseError,
};
use futures_util::{future::LocalBoxFuture, StreamExt};
use parcel_common::api_types::EncryptedData;

#[derive(Debug, thiserror::Error)]
enum EncryptionError {
    InvalidUtf8Body(Utf8Error),
    InvalidJsonData(serde_json::Error),
    InvalidAesData(anyhow::Error),
    EncryptResponseError(anyhow::Error),
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
            EncryptionError::InvalidAesData(err) => {
                write!(f, "The content body could not be decrypted: {:?}", err)
            }
            EncryptionError::EncryptResponseError(err) => {
                write!(
                    f,
                    "An internal error occured when encrypting response data: {}",
                    err
                )
            }
        }
    }
}

impl ResponseError for EncryptionError {
    fn status_code(&self) -> actix_http::StatusCode {
        match self {
            EncryptionError::EncryptResponseError(_) => {
                actix_http::StatusCode::INTERNAL_SERVER_ERROR
            }
            _ => actix_http::StatusCode::BAD_REQUEST,
        }
    }
}

#[derive(Default)]
pub struct DataEncryption {
    /// If true, then the request/response will only be decrypted/encrypted if Use-Decryption/Use-Encryption headers are true.
    /// If the headers are not present they will be encrypted/decrypted by default.
    pub optional_encryption: bool,
}

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
            optional_encryption: self.optional_encryption,
        }))
    }
}

pub struct DataEncryptionMiddleware<S> {
    service: Arc<S>,
    optional_encryption: bool,
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
        let optional_encryption = self.optional_encryption;

        Box::pin(async move {
            let use_decryption = match optional_encryption {
                true => {
                    let use_decryption_header = req.headers().get("Use-Decryption");

                    if let Some(use_decryption_header) = use_decryption_header {
                        use_decryption_header
                            .to_str()
                            .unwrap_or("true")
                            .parse::<bool>()
                            .unwrap_or(true)
                    } else {
                        true
                    }
                }
                false => true,
            };

            if use_decryption {
                let mut payload = req.take_payload();
                let mut req_body = Vec::with_capacity(1024);
                while let Some(chunk) = payload.next().await {
                    req_body.extend_from_slice(&chunk?);
                }

                if !req_body.is_empty() {
                    let body = core::str::from_utf8(&req_body).map(|body| body.trim());

                    match body {
                        Ok(body) => {
                            // parse to EncryptedData struct
                            match serde_json::from_str::<EncryptedData>(body) {
                                Ok(encrypted_data) => {
                                    if let Some(data) = encrypted_data.data {
                                        // decrypt data
                                        let decrypted_string =
                                            parcel_common::aes::decrypt_json_data(&data);

                                        match decrypted_string {
                                            Ok(decrypted_string) => {
                                                // set request payload to decrypted string
                                                let (_, mut payload) =
                                                    actix_http::h1::Payload::create(true);

                                                let payload_data =
                                                    BytesMut::from(decrypted_string.as_bytes());
                                                payload.unread_data(payload_data.into());
                                                req.set_payload(payload.into());
                                            }
                                            Err(err) => {
                                                return Ok(req
                                                    .error_response(
                                                        EncryptionError::InvalidAesData(err),
                                                    )
                                                    .map_into_right_body())
                                            }
                                        }
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

            let use_encryption = match optional_encryption {
                true => {
                    let use_encryption_header = req.headers().get("Use-Encryption");

                    if let Some(use_encryption_header) = use_encryption_header {
                        use_encryption_header
                            .to_str()
                            .unwrap_or("true")
                            .parse::<bool>()
                            .unwrap_or(true)
                    } else {
                        true
                    }
                }
                false => true,
            };

            let service_response = service.call(req).await?;

            // Don't encrypt errors. No information will be leaked because errors are caught by the WrapErrors middleware and if necessary, are made opaque
            if service_response.response().error().is_some() {
                return Ok(service_response.map_into_left_body());
            }

            if use_encryption {
                // get body from response (if any)
                let (req, res) = service_response.into_parts();
                let (res, body) = res.into_parts();
                let bytes = actix_http::body::to_bytes(body).await;

                match bytes {
                    Ok(bytes) => {
                        if !bytes.is_empty() {
                            // encrypt body, create EncryptedData and serialize to json
                            let encrypted_data = parcel_common::aes::encrypt_json_data(&bytes);
                            let body_json = serde_json::to_vec(&EncryptedData {
                                data: Some(encrypted_data),
                            })
                            .unwrap();

                            // set json string as response payload
                            let res = res.set_body(BoxBody::new(Bytes::from(body_json)));
                            let result = ServiceResponse::new(req, res).map_into_right_body();
                            Ok(result)
                        } else {
                            // if body is empty, return empty body response
                            let res = res.set_body(BoxBody::new(bytes));
                            let result = ServiceResponse::new(req, res).map_into_right_body();
                            Ok(result)
                        }
                    }
                    Err(_) => {
                        unimplemented!("to_bytes should never fail");
                    }
                }
            } else {
                Ok(service_response.map_into_left_body())
            }
        })
    }
}
