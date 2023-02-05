use std::fmt::Display;

use actix_web::{
    post,
    web::{Data, Query},
    HttpResponse, Responder, ResponseError,
};
use parcel_common::api_types::auth::Provider;
use serde::Deserialize;

use crate::data::steam::Steam;

#[derive(Debug, Deserialize)]
pub struct AuthQuery {
    provider: Provider,
    display_name: String,
    code: String,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    UnsupportedPlatform,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnsupportedPlatform => write!(f, "The provided platform is not supported"),
        }
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> actix_http::StatusCode {
        actix_http::StatusCode::INTERNAL_SERVER_ERROR
    }
}

#[post("auth/ds")]
pub async fn auth(request: Query<AuthQuery>, steam: Data<Steam>) -> Result<impl Responder, Error> {
    match &request.provider {
        Provider::Steam => {
            // todo
            Ok(HttpResponse::InternalServerError().body("not implemented"))
        }
        _ => Err(Error::UnsupportedPlatform),
    }
}
