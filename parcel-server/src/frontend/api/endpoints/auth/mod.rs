use actix_web::{
    get, post,
    web::{Data, Json, Redirect},
    HttpRequest,
};
use parcel_common::api_types::auth::Provider;
use steam_auth::Redirector;

use crate::{
    data::platforms::steam::Steam,
    frontend::{
        api::models::auth::{
            AuthRequest, CheckAuthRequest, CheckAuthResponse, InitAuthResponse,
            SteamAuthCallbackResponse,
        },
        error::ApiError,
        result::{ApiResponse, ApiResult},
    },
};

#[post("auth")]
pub async fn auth(
    request: Json<AuthRequest>,
    http_request: HttpRequest,
) -> ApiResult<InitAuthResponse> {
    match request.provider {
        Provider::Steam => {
            let redirector = Redirector::new(
                &get_site_url(&http_request),
                "/frontend/api/auth/steam/callback",
            )
            .map_err(|e| ApiError::Internal(anyhow::anyhow!("Failed to create redirector: {e}")))?;

            let redirect_url = redirector.url();

            Ok(ApiResponse::ok(InitAuthResponse {
                redirect_url: redirect_url.to_string(),
                token: "".into(),
            }))
        }
        Provider::Epic => Err(anyhow::anyhow!("Epic auth not implemented").into()),
    }
}

#[get("auth/check")]
pub async fn check_auth(request: Json<CheckAuthRequest>) -> ApiResult<CheckAuthResponse> {
    Err(anyhow::anyhow!("Not implemented").into())
}

#[get("auth/steam/callback")]
pub async fn steam_callback(
    request: HttpRequest,
    steam: Data<Steam>,
) -> Result<Redirect, ApiError> {
    Ok(Redirect::to("/frontend/login/success"))
}

fn get_site_url(request: &HttpRequest) -> String {
    let uri = request.connection_info();

    dbg!(format!("{}://{}", uri.scheme(), uri.host()))
}
