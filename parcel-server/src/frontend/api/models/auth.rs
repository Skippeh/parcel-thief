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
    Success {
        auth_token: String,
        display_name: String,
        avatar_url: String,
    },
    Failure {
        error: String,
    },
}
