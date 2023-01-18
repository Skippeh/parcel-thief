use serde::{Deserialize, Serialize};

use crate::api_types::player_profile::BasicPlayerProfile;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetPlayerProfileRequest {
    #[serde(rename = "b")]
    pub basic: BasicPlayerProfile,
    /// This is seemingly always an empty string
    #[serde(rename = "id")]
    pub account_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetPlayerProfileResponse {
    #[serde(rename = "b")]
    pub basic: BasicPlayerProfile,
    #[serde(rename = "id")]
    pub account_id: String,
}
