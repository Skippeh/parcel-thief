use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteMissionsRequest {
    #[serde(rename = "missions")]
    pub missions: Vec<Mission>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mission {
    /// The account id of the person who created the mission
    #[serde(rename = "cr")]
    pub creator_account_id: String,
    /// The online id of the mission
    #[serde(rename = "id")]
    pub mission_online_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteMissionsResponse;
