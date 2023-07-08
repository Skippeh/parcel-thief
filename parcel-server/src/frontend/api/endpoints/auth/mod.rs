use actix_web::{
    get, post,
    web::{Data, Json, Redirect},
    HttpRequest,
};
use anyhow::Context;
use chrono::Utc;
use jwt::SignWithKey;
use parcel_common::api_types::auth::Provider;
use steam_auth::{Redirector, Verifier};

use crate::{
    data::{
        database::Database, jwt::JwtSecret, memory_cache::MemoryCache, platforms::steam::Steam,
    },
    frontend::{
        api::models::auth::{
            AuthRequest, CheckAuthRequest, CheckAuthResponse, InitAuthResponse, JwtPayload,
        },
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

            Ok(ApiResponse::ok(InitAuthResponse {
                redirect_url: redirect_url.to_string(),
            }))
        }
        Provider::Epic => Err(anyhow::anyhow!("Epic auth not implemented").into()),
    }
}

#[post("auth/check")]
pub async fn check_auth(
    request: Json<CheckAuthRequest>,
    auth_cache: Data<FrontendAuthCache>,
) -> ApiResult<CheckAuthResponse> {
    let auth_response = auth_cache.get(&request.callback_token);

    if let Some(auth_response) = auth_response {
        Ok(ApiResponse::ok(auth_response))
    } else {
        Ok(ApiResponse::ok(CheckAuthResponse::Failure {
            error: "Login has expired".into(),
        }))
    }
}

#[get("auth/callback/steam")]
pub async fn steam_callback(
    request: HttpRequest,
    database: Data<Database>,
    steam: Data<Steam>,
    auth_cache: Data<FrontendAuthCache>,
    jwt_secret: Data<JwtSecret>,
) -> Result<Redirect, ApiError> {
    let (request, verifier) =
        Verifier::from_querystring(&request.query_string()).map_err(anyhow::Error::msg)?;

    let (parts, body) = request.into_parts();

    let client = reqwest::Client::new();
    let response = client
        .post(&parts.uri.to_string())
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .context("Failed to send request to verify steam callback")?;

    let text = response
        .text()
        .await
        .context("Failed to read steam callback response")?;

    let response_token = generate_response_token();
    let response = match verifier.verify_response(text) {
        Ok(steam_id) => {
            let conn = database.connect()?;
            let accounts = conn.accounts();

            let account = accounts
                .get_by_provider_id(Provider::Steam, &steam_id.to_string())
                .await?;

            match account {
                Some(account) => {
                    let user_summary = steam
                        .get_player_summaries(&[&steam_id])
                        .await?
                        .remove(&steam_id)
                        .context("Failed to get user summary")?;

                    let payload = JwtPayload {
                        expires_at: (Utc::now() + chrono::Duration::days(7)).timestamp(),
                        account_id: account.id,
                    };

                    let auth_token = payload
                        .sign_with_key(jwt_secret.as_ref())
                        .map_err(anyhow::Error::msg)?;

                    CheckAuthResponse::Success {
                        auth_token,
                        avatar_url: user_summary.avatar_full,
                        name: user_summary.name,
                    }
                }
                None => {
                    // set error to account not found
                    // or should the account be created?
                    CheckAuthResponse::Failure {
                        error: "Account not found, log in to the game server first".into(),
                    }
                }
            }
        }
        Err(e) => {
            log::error!("Failed to verify steam callback: {}", e);

            CheckAuthResponse::Failure {
                error: "Failed to verify authentication".into(),
            }
        }
    };

    auth_cache.insert(response_token.clone(), response).await;

    Ok(Redirect::to(format!(
        "/frontend/login?callback_token={}",
        response_token
    )))
}

fn get_site_url(request: &HttpRequest) -> String {
    let uri = request.connection_info();

    dbg!(format!("{}://{}", uri.scheme(), uri.host()))
}

fn generate_response_token() -> String {
    //uuid::Uuid::new_v4().to_string()
    parcel_common::rand::generate_string(
        64,
        b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ",
    )
}
