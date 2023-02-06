use std::fmt::Display;

use actix_http::StatusCode;
use actix_web::{
    get,
    web::{Data, Query},
    HttpResponse, Responder,
};
use parcel_common::api_types::auth::Provider;
use serde::Deserialize;

use crate::{
    data::steam::{Steam, VerifyUserAuthTicketError},
    response_error::{impl_response_error, CommonResponseError},
};

#[derive(Debug, Deserialize)]
pub struct AuthQuery {
    provider: Provider,
    #[serde(rename = "display_name")]
    _display_name: String,
    code: String,
}

#[allow(clippy::enum_variant_names)]
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

impl_response_error!(Error);
impl CommonResponseError for Error {
    fn get_status_code(&self) -> String {
        match self {
            Error::UnsupportedPlatform => "AU-UP",
            Error::ApiResponseError(_) => "AU-AE",
            Error::InvalidCode => "AU-IC",
        }
        .into()
    }

    fn get_http_status_code(&self) -> StatusCode {
        match self {
            Error::UnsupportedPlatform => StatusCode::BAD_REQUEST,
            Error::ApiResponseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::InvalidCode => StatusCode::UNAUTHORIZED,
        }
    }

    fn get_message(&self) -> String {
        match self {
            Error::UnsupportedPlatform => "unsupported provider",
            Error::ApiResponseError(_) => "provider error",
            Error::InvalidCode => "invalid provider code",
        }
        .into()
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
