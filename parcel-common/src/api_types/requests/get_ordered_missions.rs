use serde::{Deserialize, Serialize};

use crate::api_types::mission::Mission;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetOrderedMissionsRequest;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetOrderedMissionsResponse {
    #[serde(rename = "missions")]
    pub missions: Vec<Mission>,
}
