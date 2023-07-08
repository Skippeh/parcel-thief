use chrono::Utc;
use parcel_common::api_types::auth::Provider;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthRequest {
    pub provider: Provider,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InitAuthResponse {
    pub redirect_url: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckAuthRequest {
    pub callback_token: String,
}

#[derive(Debug, serde::Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum CheckAuthResponse {
    #[serde(rename_all = "camelCase")]
    Success {
        name: String,
        avatar_url: String,
        auth_token: String,
    },
    Failure {
        error: String,
    },
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JwtPayload {
    pub expires_at: i64,
    pub account_id: String,
}
