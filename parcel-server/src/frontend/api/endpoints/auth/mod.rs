use actix_web::{get, post, web::Json};

use crate::frontend::{
    api::models::auth::{AuthRequest, CheckAuthRequest, CheckAuthResponse, InitAuthResponse},
    result::ApiResult,
};

#[post("auth")]
pub async fn auth(request: Json<AuthRequest>) -> ApiResult<InitAuthResponse> {
    Err(anyhow::anyhow!("Not implemented").into())
}

#[get("auth/check")]
pub async fn check_auth(request: Json<CheckAuthRequest>) -> ApiResult<CheckAuthResponse> {
    Err(anyhow::anyhow!("Not implemented").into())
}
