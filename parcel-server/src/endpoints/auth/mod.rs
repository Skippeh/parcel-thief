use std::fmt::Display;

use actix_http::StatusCode;
use actix_web::{
    get,
    web::{Data, Query},
    HttpResponse, Responder,
};
use chrono::{Days, Utc};
use parcel_common::api_types::auth::Provider;
use rand::{seq::SliceRandom, SeedableRng};
use rand_chacha::ChaCha20Rng;
use serde::Deserialize;

use crate::{
    data::{
        session::DsSession,
        steam::{Steam, VerifyUserAuthTicketError},
    },
    response_error::{impl_response_error, CommonResponseError},
    session::{redis::RedisSessionStore, Session},
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
    UnsupportedPlatform(Provider),
    ApiResponseError(anyhow::Error),
    InvalidCode,
    InternalError(anyhow::Error),
    AlreadyAuthenticated,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnsupportedPlatform(platform) => {
                write!(f, "The provided platform is not supported: {:?}", platform)
            }
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
            Error::InternalError(err) => {
                write!(f, "An internal error occured: {:?}", err)
            }
            Error::AlreadyAuthenticated => write!(f, "The user is already authenticated"),
        }
    }
}

impl_response_error!(Error);
impl CommonResponseError for Error {
    fn get_status_code(&self) -> String {
        match self {
            Error::UnsupportedPlatform(_) => "AU-UP",
            Error::ApiResponseError(_) => "AU-AE",
            Error::InvalidCode => "AU-IC",
            Error::InternalError(_) => "AU_IE",
            Error::AlreadyAuthenticated => "AU_AA",
        }
        .into()
    }

    fn get_http_status_code(&self) -> StatusCode {
        match self {
            Error::UnsupportedPlatform(_) => StatusCode::BAD_REQUEST,
            Error::ApiResponseError(_) | Error::InternalError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Error::InvalidCode => StatusCode::UNAUTHORIZED,
            Error::AlreadyAuthenticated => StatusCode::BAD_REQUEST,
        }
    }

    fn get_message(&self) -> String {
        match self {
            Error::UnsupportedPlatform(_) => "unsupported provider",
            Error::ApiResponseError(_) => "provider error",
            Error::InvalidCode => "invalid provider code",
            Error::InternalError(_) => "internal error",
            Error::AlreadyAuthenticated => "already authenticated",
        }
        .into()
    }
}

#[get("auth/ds")]
pub async fn auth(
    request: Query<AuthQuery>,
    steam: Data<Steam>,
    session_store: Data<RedisSessionStore>,
) -> Result<impl Responder, Error> {
    match &request.provider {
        Provider::Steam => {
            /*let mut session = Session::new(
                generate_token(),
                Utc::now().checked_add_days(Days::new(1)).unwrap(),
            );

            session.set_raw("account_id", "test");

            session_store
                .save_session(&session)
                .await
                .map_err(|err| Error::InternalError(err.into()))?;*/

            let user_info = steam
                .verify_user_auth_ticket(&request.code)
                .await
                .map_err(|err| match err {
                    VerifyUserAuthTicketError::InvalidTicket => Error::InvalidCode,
                    other => Error::ApiResponseError(other.into()),
                })?;

            // todo: expire current session (if any)

            // todo: create account if one doesn't exist

            // todo: create new session

            Ok(HttpResponse::InternalServerError().body("not implemented"))
        }
        other => Err(Error::UnsupportedPlatform(other.clone())),
    }
}

fn generate_token() -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#&/()=?\\";
    let mut rng = ChaCha20Rng::from_entropy();
    let mut bytes = Vec::with_capacity(64);

    for _ in 0..64 {
        bytes.push(*CHARS.choose(&mut rng).unwrap());
    }

    unsafe { String::from_utf8_unchecked(bytes) }
}
