use serde::{Deserialize, Serialize};

use crate::api_types::{area::AreaHash, mission::Mission};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FindMissionsRequest {
    #[serde(rename = "aid")]
    pub area_hash: AreaHash,
    #[serde(rename = "limit")]
    pub limit: u32,
    #[serde(rename = "lbp")]
    pub limit_pot_baggages: u32,
    #[serde(rename = "mlpp")]
    pub mission_limit_per_pot: u32,
    #[serde(rename = "plpp")]
    pub private_limit_per_pot: u32,
    /// The locations that we want to find missions in.
    #[serde(rename = "qids")]
    pub qpid_ids: Vec<i32>,
    /// Player account ids that should be prioritized.
    /// These are the account ids of the strand contracts the player has.
    #[serde(rename = "targetIds")]
    pub target_ids: Vec<String>,
    /// No clue what this is for, but the value is seemingly always 60.
    #[serde(rename = "targetRate")]
    pub target_rate: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FindMissionsResponse {
    #[serde(rename = "missions")]
    missions: Vec<Mission>,
}
