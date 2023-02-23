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
    /// The id is prefixed by an identifier, and separated by a '_'.
    ///
    /// The id prefix can be of multiple types, including (but maybe not limited to):
    /// - zygo_ - player account id
    /// - steam_ - steam account (steamid64)
    /// - epic_ - epic account id
    #[serde(rename = "id")]
    pub id: String,
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

    /// The original id that was given by the request (so not necessarily an account id)
    #[serde(rename = "id")]
    pub id: String,
}
