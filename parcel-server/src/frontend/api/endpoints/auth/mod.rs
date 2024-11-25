pub mod local;
pub mod steam;

use std::path::Path;

use actix_web::{
    post,
    web::{Data, Json},
    HttpRequest,
};
use chrono::{DateTime, Utc};
use jwt::SignWithKey;
use parcel_common::api_types::{
    auth::Provider,
    frontend::auth::{
        AuthRequest, CheckAuthRequest, CheckAuthResponse, InitAuthResponse, JwtPayload,
    },
};
use steam_auth::Redirector;

use crate::{
    data::{
        database::Database,
        jwt_secret::JwtSecret,
        memory_cache::{MemoryCache, PersistentCache},
    },
    db::models::frontend_account::FrontendAccount,
    endpoints::EmptyResponse,
    frontend::{
        error::ApiError,
        jwt_session::{JwtSession, SessionBlacklistCache, BLACKLIST_CACHE_PATH},
        result::{ApiResponse, ApiResult},
    },
};

pub type FrontendAuthCache = MemoryCache<String, CheckAuthResponse>;

#[post("auth")]
pub async fn auth(
    request: Json<AuthRequest>,
    http_request: HttpRequest,
) -> ApiResult<InitAuthResponse> {
    match request.provider {
        Provider::Steam => {
            let redirector = Redirector::new(
                get_site_url(&http_request),
                "/frontend/api/auth/callback/steam",
            )
            .map_err(|e| ApiError::Internal(anyhow::anyhow!("Failed to create redirector: {e}")))?;

            let redirect_url = redirector.url();

            ApiResponse::ok(InitAuthResponse {
                redirect_url: redirect_url.to_string(),
            })
        }
        Provider::Epic => {
            Err(anyhow::anyhow!("Epic auth is not implemented, use a local account").into())
        }
        Provider::Server => {
            Err(anyhow::anyhow!("Players can not authenticate a server account").into())
        }
    }
}

#[post("auth/check")]
pub async fn check_auth(
    request: Json<CheckAuthRequest>,
    auth_cache: Data<FrontendAuthCache>,
) -> ApiResult<CheckAuthResponse> {
    let auth_response = auth_cache.get(&request.callback_token);

    if let Some(auth_response) = auth_response {
        ApiResponse::ok(auth_response)
    } else {
        ApiResponse::ok(CheckAuthResponse::Failure {
            error: "Login has expired".into(),
        })
    }
}

#[post("auth/logout")]
pub async fn logout(
    session: JwtSession,
    session_blacklist_cache: Data<SessionBlacklistCache>,
    database: Data<Database>,
) -> ApiResult<EmptyResponse> {
    database
        .connect()
        .await?
        .frontend_accounts()
        .delete_session_by_token(&session.token)
        .await?;

    let expiry_time = chrono::DateTime::from_timestamp(session.expires_at, 0)
        .ok_or_else(|| anyhow::anyhow!("Invalid expiry time"))?;
    session_blacklist_cache
        .insert(session.token, expiry_time)
        .await;
    session_blacklist_cache
        .save_to_file(Path::new(BLACKLIST_CACHE_PATH))
        .await
        .map_err(|err| ApiError::Internal(anyhow::anyhow!(err)))?;

    ApiResponse::ok(EmptyResponse)
}

fn get_site_url(request: &HttpRequest) -> String {
    let uri = request.connection_info();

    format!("{}://{}", uri.scheme(), uri.host())
}

fn generate_response_token() -> String {
    parcel_common::rand::generate_string(
        64,
        b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ",
    )
}

fn create_auth_token(
    account: &FrontendAccount,
    jwt_secret: &JwtSecret,
) -> Result<(String, DateTime<Utc>), jwt::error::Error> {
    let expire_date = Utc::now() + chrono::Duration::days(7);
    let payload = JwtPayload {
        expires_at: expire_date.timestamp(),
        account_id: account.id,
    };

    let auth_token = payload.sign_with_key(jwt_secret)?;
    Ok((auth_token, expire_date))
}
