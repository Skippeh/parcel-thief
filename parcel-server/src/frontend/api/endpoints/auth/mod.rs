use actix_web::{
    get, post,
    web::{Data, Json, Redirect},
    HttpRequest,
};
use anyhow::Context;
use parcel_common::api_types::auth::Provider;
use steam_auth::{Redirector, Verifier};

use crate::{
    data::{database::Database, memory_cache::MemoryCache, platforms::steam::Steam},
    frontend::{
        api::models::auth::{AuthRequest, CheckAuthRequest, CheckAuthResponse, InitAuthResponse},
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

#[get("auth/check")]
pub async fn check_auth(
    request: Json<CheckAuthRequest>,
    auth_cache: Data<FrontendAuthCache>,
) -> ApiResult<CheckAuthResponse> {
    Err(anyhow::anyhow!("Not implemented").into())
}

#[get("auth/callback/steam")]
pub async fn steam_callback(
    request: HttpRequest,
    database: Data<Database>,
    steam: Data<Steam>,
    auth_cache: Data<FrontendAuthCache>,
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
                    // todo
                    CheckAuthResponse::Failure {
                        error: "not implemented".into(),
                    }
                }
                None => {
                    // set error to account not found
                    // or should the account be created?
                    CheckAuthResponse::Failure {
                        error: "account not found".into(),
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
