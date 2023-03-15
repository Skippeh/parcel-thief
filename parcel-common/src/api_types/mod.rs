use serde::{Deserialize, Serialize};

pub mod area;
pub mod auth;
pub mod mission;
pub mod object;
pub mod player_profile;
pub mod rank;
pub mod requests;
pub mod road;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EncryptedData {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}
