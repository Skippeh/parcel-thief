use std::fmt::Display;

use actix_http::StatusCode;
use actix_web::{get, web::Data, HttpResponse, Responder};
use parcel_common::api_types::auth::{AuthResponse, SessionInfo, SessionProperties, UserInfo};
use redis::RedisError;

use crate::{
    data::database::Database,
    response_error::{impl_response_error, CommonResponseError},
    session::Session,
    GatewayUrl,
};

#[derive(Debug)]
pub struct Error(anyhow::Error);

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<anyhow::Error> for Error {
    fn from(value: anyhow::Error) -> Self {
        Self(value)
    }
}

impl From<RedisError> for Error {
    fn from(value: RedisError) -> Self {
        Self(value.into())
    }
}

impl From<diesel::ConnectionError> for Error {
    fn from(value: diesel::ConnectionError) -> Self {
        Self(value.into())
    }
}

impl_response_error!(Error);
impl CommonResponseError for Error {
    fn get_status_code(&self) -> String {
        "SV-IE".into()
    }

    fn get_http_status_code(&self) -> actix_http::StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn get_message(&self) -> String {
        "internal error".into()
    }
}

/// This route doesn't exist on the "real" ds server, it's only used for debugging.
///
/// Leaving it in for production is safe since the same info is sent to the client on auth.
#[get("auth/me")]
pub async fn me(
    session: Session,
    database: Data<Database>,
    gateway_url: Data<GatewayUrl>,
) -> Result<impl Responder, Error> {
    let db = database.connect()?;
    let accounts = db.accounts();
    let account = accounts
        .get_by_provider_id(session.provider, &session.provider_id)
        .await
        .map_err(|err| Error(err.into()))?
        .unwrap(); // This is safe since a session can't exist without an account

    let response = AuthResponse {
        session: SessionInfo {
            gateway: gateway_url.as_ref().clone().into(),
            token: session.get_token().to_owned(),
            properties: SessionProperties {
                last_login: account.last_login_date.timestamp(),
            },
        },
        user: UserInfo {
            display_name: account.display_name,
            id: account.id,
            provider: account.provider,
        },
    };

    Ok(HttpResponse::Ok().json(response))
}
