pub mod local;
pub mod steam;

use actix_web::{
    post,
    web::{Data, Json},
    HttpRequest,
};
use chrono::Utc;
use flagset::FlagSet;
use jwt::SignWithKey;
use parcel_common::api_types::{
    auth::Provider,
    frontend::auth::{
        AuthRequest, CheckAuthRequest, CheckAuthResponse, FrontendPermissions, InitAuthResponse,
        JwtPayload,
    },
};
use steam_auth::Redirector;

use crate::{
    data::{jwt_secret::JwtSecret, memory_cache::MemoryCache},
    db::models::frontend_account::FrontendAccount,
    frontend::{
        error::ApiError,
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
                &get_site_url(&http_request),
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
) -> Result<String, jwt::error::Error> {
    let permissions = FlagSet::<FrontendPermissions>::new_truncated(account.permissions);
    let payload = JwtPayload {
        expires_at: (Utc::now() + chrono::Duration::days(7)).timestamp(),
        account_id: account.id,
        permissions: permissions.bits(),
    };

    let auth_token = payload.sign_with_key(jwt_secret)?;
    Ok(auth_token)
}
