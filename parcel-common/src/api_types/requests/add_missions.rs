use serde::{Deserialize, Serialize};

use crate::api_types::{
    area::AreaHash,
    mission::{
        Baggage, DynamicLocationInfo, DynamicMissionInfo, Mission, MissionType, OnlineMissionType,
        SupplyInfo,
    },
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct NewMission {
    #[serde(rename = "aid")]
    pub area_hash: AreaHash,
    #[serde(rename = "qid")]
    pub qpid_id: i32,
    #[serde(rename = "sid")]
    pub qpid_start_location: i32,
    #[serde(rename = "eid")]
    pub qpid_end_location: i32,
    #[serde(rename = "mid")]
    pub mission_static_id: i64,
    #[serde(rename = "mt")]
    pub mission_type: MissionType,
    #[serde(rename = "omt")]
    pub online_mission_type: OnlineMissionType,
    #[serde(rename = "si")]
    pub supply_info: Option<SupplyInfo>,
    #[serde(rename = "dsi")]
    pub dynamic_start_info: Option<DynamicLocationInfo>,
    #[serde(rename = "dei")]
    pub dynamic_end_info: Option<DynamicLocationInfo>,
    #[serde(rename = "dmi")]
    pub dynamic_mission_info: Option<DynamicMissionInfo>,
    #[serde(rename = "b")]
    pub baggages: Option<Vec<Baggage>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddMissionsRequest {
    #[serde(rename = "ms")]
    pub missions: Vec<NewMission>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddMissionsResponse {
    #[serde(rename = "missions")]
    pub missions: Vec<Mission>,
}
