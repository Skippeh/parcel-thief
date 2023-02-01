use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserInfo {
    pub provider: String,
    pub id: String,
    pub display_name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SessionProperties {
    /// The epoch time in seconds of the last login (seems to always be same as the current time)
    #[serde(rename = "ll")]
    pub last_login: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SessionInfo {
    pub token: String,
    pub gateway: String,
    pub properties: SessionProperties,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AuthResponse {
    pub user: UserInfo,
    pub session: SessionInfo,
}
