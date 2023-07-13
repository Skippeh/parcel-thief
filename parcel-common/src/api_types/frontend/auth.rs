use typescript_type_def::TypeDef;

use crate::api_types::auth::Provider;

#[derive(Debug, serde::Deserialize, TypeDef)]
#[serde(rename_all = "camelCase")]
pub struct AuthRequest {
    pub provider: Provider,
}

#[derive(Debug, serde::Serialize, TypeDef)]
#[serde(rename_all = "camelCase")]
pub struct InitAuthResponse {
    pub redirect_url: String,
}

#[derive(Debug, serde::Deserialize, TypeDef)]
#[serde(rename_all = "camelCase")]
pub struct CheckAuthRequest {
    pub callback_token: String,
}

#[derive(Debug, serde::Serialize, TypeDef, Clone)]
#[serde(rename_all = "camelCase", tag = "type")]
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

#[derive(Debug, serde::Deserialize, serde::Serialize, TypeDef)]
#[serde(rename_all = "camelCase")]
pub struct JwtPayload {
    pub expires_at: i64,
    pub account_id: String,
}
