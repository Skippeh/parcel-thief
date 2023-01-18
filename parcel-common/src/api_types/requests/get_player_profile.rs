use serde::{Deserialize, Serialize};

use crate::api_types::player_profile::BasicPlayerProfile;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetPlayerProfileRequest {
    #[serde(rename = "profiles")]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetPlayerProfileResponse {
    #[serde(rename = "profiles")]
    pub profiles: Vec<ProfileResponse>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProfileResponse {
    #[serde(rename = "b")]
    pub basic: BasicPlayerProfile,
    #[serde(rename = "id")]
    pub account_id: String,
}
