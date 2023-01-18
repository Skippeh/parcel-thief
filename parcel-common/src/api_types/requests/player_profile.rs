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
pub struct SetPlayerProfileResponse<'a> {
    #[serde(rename = "b")]
    pub basic: &'a BasicPlayerProfile,
    #[serde(rename = "id")]
    pub account_id: &'a str,
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
pub struct GetPlayerProfileResponse<'vec, 'res> {
    pub profiles: &'vec Vec<ProfileResponse<'res>>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ProfileResponse<'a> {
    #[serde(rename = "b")]
    pub basic: &'a BasicPlayerProfile,
    #[serde(rename = "id")]
    pub account_id: &'a str,
}
