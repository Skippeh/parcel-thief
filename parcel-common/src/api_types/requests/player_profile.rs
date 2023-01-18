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

#[derive(Debug, Serialize, Clone)]
pub struct SetPlayerProfileResponse {
    #[serde(rename = "b")]
    pub basic: BasicPlayerProfile,
    #[serde(rename = "id")]
    pub account_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetPlayerProfileRequest {
    pub profiles: Vec<ProfileRequest>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProfileRequest {
    /// This is either flags or filter, i'm guessing flags. Value is seemingly always 1.
    #[serde(rename = "f")]
    pub flags: u64,
    #[serde(rename = "id")]
    pub account_id: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct GetPlayerProfileResponse {
    pub profiles: Vec<ProfileResponse>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ProfileResponse {
    #[serde(rename = "b")]
    pub basic: BasicPlayerProfile,
    #[serde(rename = "id")]
    pub account_id: String,
}
