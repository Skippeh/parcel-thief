use serde::{Deserialize, Serialize};

use crate::api_types::mission::{Baggage, Mission, ProgressState};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetMissionProgressRequest {
    #[serde(rename = "bgs")]
    pub baggages: Option<Vec<Baggage>>,
    #[serde(rename = "cr")]
    pub creator_account_id: String,
    #[serde(rename = "id")]
    pub mission_online_id: String,
    #[serde(rename = "did")]
    pub delivered_location_id: i32,
    #[serde(rename = "pr")]
    pub progress_state: ProgressState,
    #[serde(rename = "qid")]
    pub qpid_id: i32,
}

pub type SetMissionProgressResponse = Mission;
