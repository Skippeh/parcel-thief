use serde::{Deserialize, Serialize};

use crate::api_types::auth::Provider;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReverseLookupRequest {
    /// A list of provider user ids, on steam this is the steamid64, and on epic it's the unique user id.
    #[serde(rename = "bs")]
    pub provider_account_ids: Vec<String>,
    #[serde(rename = "p")]
    pub provider: Provider,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReverseLookupResponse {
    // unknown, but likely a string array of account ids
    #[serde(rename = "ids")]
    pub account_ids: Vec<String>,
}
