use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetVersionRequest;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetVersionResponse {
    /// Is normally "prod"
    #[serde(rename = "domain")]
    pub domain: String,
    #[serde(rename = "major")]
    pub major: u32,
    #[serde(rename = "minor")]
    pub minor: u32,
    #[serde(rename = "version")]
    pub version: String,
    /// Is normally "ds", should probably always be
    #[serde(rename = "zone")]
    zone: String,
}

impl GetVersionResponse {
    /// Gets the version that matches the real game server as of 2023-01-18
    pub fn current_version() -> Self {
        Self {
            domain: "prod".into(),
            major: 0,
            minor: 12,
            version: "0.0.1".into(),
            zone: "ds".into(),
        }
    }
}
