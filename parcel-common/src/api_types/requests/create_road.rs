use serde::{Deserialize, Serialize};

use crate::api_types::{area::AreaHash, road::Road};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateRoadRequest {
    #[serde(rename = "aid")]
    pub area_hash: AreaHash,
    #[serde(rename = "data")]
    pub data: String,
    #[serde(rename = "ver")]
    pub data_version: i32,
    #[serde(rename = "sid")]
    pub start_location_id: i32,
    #[serde(rename = "eid")]
    pub end_location_id: i32,
    #[serde(rename = "sq")]
    pub start_qpid_id: i32,
    #[serde(rename = "eq")]
    pub end_qpid_id: i32,
    #[serde(rename = "hdif")]
    pub max_height_difference: u32,
    #[serde(rename = "len")]
    pub path_length: u32,
}

pub type CreateRoadResponse = Road;
