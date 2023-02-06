use std::fmt::Display;

use actix_http::StatusCode;
use actix_web::{
    get,
    web::{Data, Query},
    HttpResponse, Responder, ResponseError,
};
use parcel_common::api_types::auth::Provider;
use serde::Deserialize;

use crate::data::steam::{Steam, VerifyUserAuthTicketError};

#[derive(Debug, Deserialize)]
pub struct AuthQuery {
    provider: Provider,
    #[serde(rename = "display_name")]
    _display_name: String,
    code: String,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    UnsupportedPlatform,
    ApiResponseError(anyhow::Error),
    InvalidCode,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnsupportedPlatform => write!(f, "The provided platform is not supported"),
            Error::ApiResponseError(err) => {
                write!(
                    f,
                    "An error occured while sending a request to an external api: {:?}",
                    err
                )
            }
            Error::InvalidCode => {
                write!(f, "Could not authenticate user from code")
            }
        }
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::UnsupportedPlatform => StatusCode::BAD_REQUEST,
            Error::ApiResponseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::InvalidCode => StatusCode::UNAUTHORIZED,
        }
    }
}

#[get("auth/ds")]
pub async fn auth(request: Query<AuthQuery>, steam: Data<Steam>) -> Result<impl Responder, Error> {
    match &request.provider {
        Provider::Steam => {
            let user_info = steam
                .verify_user_auth_ticket(&request.code)
                .await
                .map_err(|err| match err {
                    VerifyUserAuthTicketError::InvalidTicket => Error::InvalidCode,
                    other => Error::ApiResponseError(other.into()),
                })?;

            Ok(HttpResponse::InternalServerError().body("not implemented"))
        }
        _ => Err(Error::UnsupportedPlatform),
    }
}
