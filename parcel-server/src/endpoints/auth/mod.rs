pub mod me;

use std::fmt::Display;

use actix_http::StatusCode;
use actix_web::{
    get,
    web::{Data, Json, Query},
    HttpRequest,
};

use chrono::Utc;
use parcel_common::{
    api_types::auth::{AuthResponse, Provider, SessionInfo, SessionProperties, UserInfo},
    rand,
};
use serde::Deserialize;

use crate::{
    data::{
        database::Database,
        platforms::{
            epic::{self, Epic},
            steam::{self, Steam},
        },
        session_store::SessionStore,
    },
    response_error::{impl_response_error, CommonResponseError},
    session::Session,
    GatewayUrl, ServerSettings, WhitelistSettings,
};

use super::InternalError;

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
    ApiResponseError(anyhow::Error),
    InvalidCode,
    InternalError(InternalError),
    NotWhitelisted,
    UnsupportedProvider,
}

impl From<crate::db::QueryError> for Error {
    fn from(value: crate::db::QueryError) -> Self {
        Self::InternalError(value.into())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
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
                write!(f, "{}", err)
            }
            Error::NotWhitelisted => {
                write!(f, "Account is not whitelisted")
            }
            Error::UnsupportedProvider => {
                write!(f, "Unsupported provider")
            }
        }
    }
}

impl_response_error!(Error);
impl CommonResponseError for Error {
    fn get_status_code(&self) -> String {
        match self {
            Error::ApiResponseError(_) => "AU-AE".into(),
            Error::InvalidCode => "AU-IC".into(),
            Error::InternalError(err) => err.get_status_code(),
            Error::NotWhitelisted => "AU-NW".into(),
            Error::UnsupportedProvider => "AU-UP".into(),
        }
    }

    fn get_http_status_code(&self) -> StatusCode {
        match self {
            Error::ApiResponseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::InternalError(err) => err.get_http_status_code(),
            Error::InvalidCode => StatusCode::FORBIDDEN,
            Error::NotWhitelisted => StatusCode::FORBIDDEN,
            Error::UnsupportedProvider => StatusCode::BAD_REQUEST,
        }
    }

    fn get_message(&self) -> String {
        match self {
            Error::ApiResponseError(_) => "provider error".into(),
            Error::InvalidCode => "invalid provider code".into(),
            Error::InternalError(err) => err.get_message(),
            Error::NotWhitelisted => "not whitelisted".into(),
            Error::UnsupportedProvider => "unsupported provider".into(),
        }
    }
}

#[allow(clippy::too_many_arguments)]
#[get("auth/ds")]
pub async fn auth(
    request: Query<AuthQuery>,
    steam: Data<Steam>,
    epic: Data<Epic>,
    session_store: Data<SessionStore>,
    db: Data<Database>,
    gateway_url: Data<Option<GatewayUrl>>,
    http_request: HttpRequest,
    server_settings: Data<ServerSettings>,
    whitelist: Data<WhitelistSettings>,
) -> Result<Json<AuthResponse>, Error> {
    let provider;
    let provider_id;
    let display_name;

    match &request.provider {
        Provider::Server => {
            // Players can't auth as a server account
            return Err(Error::UnsupportedProvider);
        }
        Provider::Steam => {
            let user_id = steam
                .verify_user_auth_ticket(&request.code)
                .await
                .map_err(|err| match err {
                    steam::VerifyUserAuthTicketError::InvalidTicket => Error::InvalidCode,
                    other => Error::ApiResponseError(other.into()),
                })?;

            if !server_settings.read().await.public_server
                && !whitelist
                    .read()
                    .await
                    .is_whitelisted(&user_id.steam_id.to_string())
            {
                log::info!("Blocked non whitelisted provider id: {}", user_id.steam_id);
                return Err(Error::NotWhitelisted);
            }

            let user_info = steam
                .get_player_summaries(&[&user_id.steam_id])
                .await
                .map_err(|err| Error::InternalError(err.into()))?
                .remove(&user_id.steam_id)
                .ok_or_else(|| {
                    Error::InternalError(
                        anyhow::anyhow!("PlayerSummary not found for provided steam id").into(),
                    )
                })?;

            provider = Provider::Steam;
            provider_id = user_info.steam_id.to_string();
            display_name = user_info.name;
        }
        Provider::Epic => {
            let account_id = epic
                .verify_token(&request.code)
                .await
                .map_err(|err| match err {
                    epic::VerifyTokenError::InvalidToken => Error::InvalidCode,
                    other => Error::ApiResponseError(other.into()),
                })?;

            if !server_settings.read().await.public_server
                && !whitelist
                    .read()
                    .await
                    .is_whitelisted(&account_id.account_id)
            {
                log::info!(
                    "Blocked non whitelisted provider id: {}",
                    account_id.account_id
                );
                return Err(Error::NotWhitelisted);
            }

            let account_info = epic
                .get_account_infos(&request.code, &[&account_id.account_id])
                .await
                .map_err(|err| Error::InternalError(err.into()))?
                .into_iter()
                .next()
                .map(|tuple| tuple.1)
                .ok_or_else(|| {
                    Error::InternalError(anyhow::anyhow!("Could not query account info").into())
                })?;

            provider = Provider::Epic;
            provider_id = account_id.account_id;
            display_name = account_info.display_name;
        }
    }

    let login_date = Utc::now().naive_utc();

    if let Some(token) = session_store.find_active_session_token(provider, &provider_id) {
        session_store.delete_session(&token).await;
    }

    // create account if one doesn't exist
    let db = &mut db
        .connect()
        .await
        .map_err(|err| Error::InternalError(err.into()))?;
    let accounts = db.accounts();

    // find account for provider id, or create it if it doesn't exist yet, and also update display name
    let account = match accounts.get_by_provider_id(provider, &provider_id).await? {
        Some(account) => {
            // update display name
            accounts
                .update_display_name_and_last_login(&account.id, &display_name, &login_date)
                .await?;

            account
        }
        None => {
            // create account
            log::debug!(
                "Creating account. Provider = {:?}, Id = {}, Current display name = {}",
                provider,
                provider_id,
                display_name
            );

            accounts
                .create(provider, &provider_id, &display_name, &login_date)
                .await?
        }
    };

    // create session
    let session = Session::new(
        provider,
        &provider_id,
        &account.id,
        generate_session_token(),
    );
    let token = session.get_token().to_owned();
    session_store.save_session(session).await;

    let gateway_url = match gateway_url.as_ref() {
        Some(gateway_url) => gateway_url.0.clone(),
        None => {
            let url = infer_gateway_url(&http_request);

            log::debug!("Inferred gateway url: {}", url);

            url
        }
    };

    Ok(Json(AuthResponse {
        user: UserInfo {
            id: account.id.clone(),
            display_name: account.display_name.clone(),
            provider,
        },
        session: SessionInfo {
            token,
            gateway: gateway_url,
            properties: SessionProperties {
                last_login: login_date.and_utc().timestamp(),
            },
        },
    }))
}

fn generate_session_token() -> String {
    const CHARS: &[u8] =
        b"aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789!\"#&/()=?\\";
    rand::generate_string(64, CHARS)
}

fn infer_gateway_url(request: &HttpRequest) -> String {
    let uri = request.connection_info();

    format!("{}://{}/ds", uri.scheme(), uri.host())
}
