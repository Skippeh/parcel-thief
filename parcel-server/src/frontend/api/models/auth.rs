use parcel_common::api_types::auth::Provider;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthRequest {
    pub provider: Provider,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InitAuthResponse {
    pub token: String,
    pub oauth_url: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckAuthRequest {
    pub token: String,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckAuthResponse {
    pub auth_token: Option<String>,
    pub error: Option<String>,
}
