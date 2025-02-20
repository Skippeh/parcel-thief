use serde::{Deserialize, Serialize};

use crate::api_types::auth::Provider;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LookupRequest {
    #[serde(rename = "ids")]
    pub account_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LookupResponse {
    #[serde(rename = "users")]
    pub users: Vec<LookupUserInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LookupUserInfo {
    #[serde(rename = "a")]
    pub account_id: String,
    /// The unique id on the platform this user is on.
    /// On steam this would the the user's steamid64.
    /// On epic it's the account id.
    #[serde(rename = "b")]
    pub provider_account_id: String,
    #[serde(rename = "d")]
    pub display_name: String,
    /// The platform this player is on.
    #[serde(rename = "p")]
    pub provider: Provider,
}
