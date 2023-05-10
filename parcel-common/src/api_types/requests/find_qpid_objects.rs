use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::api_types::{
    area::AreaHash,
    object::{ObjectType, QpidObjectsResponse},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FindQpidObjectsRequest {
    #[serde(rename = "aid")]
    pub area_hash: AreaHash,
    #[serde(rename = "q")]
    pub qpid_id: i32,
    /// Account ids of people we want to prioritize finding objects from
    #[serde(rename = "u")]
    pub account_ids: Option<Vec<String>>,
    #[serde(rename = "o")]
    pub object: Option<ObjectRequest>,
    #[serde(rename = "ro")]
    pub road: Option<RoadRequest>,
    #[serde(rename = "m")]
    pub mission: Option<MissionRequest>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ObjectRequest {
    #[serde(rename = "ct")]
    pub counts: HashMap<ObjectType, i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RoadRequest {
    #[serde(rename = "ct")]
    pub count: i32,
    #[serde(rename = "plid")]
    pub prioritized_location_id: i32,
    #[serde(rename = "qe")]
    pub end_qpids: Vec<i32>,
    #[serde(rename = "rlid")]
    pub required_location_id: Option<i32>,
    #[serde(rename = "ver")]
    pub data_version: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MissionRequest {
    #[serde(rename = "areaId")]
    pub area_hash: AreaHash,
    #[serde(rename = "limit")]
    pub limit: u32,
    #[serde(rename = "qids")]
    pub qpid_ids: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FindQpidObjectsResponse {
    #[serde(rename = "n")]
    pub normal: QpidObjectsResponse,
}
