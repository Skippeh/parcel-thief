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
    pub zone: String,
}
