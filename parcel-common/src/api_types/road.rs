use serde::{Deserialize, Serialize};

use super::area::AreaHash;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Road {
    #[serde(rename = "aid")]
    pub area_hash: AreaHash,
    #[serde(rename = "cr")]
    pub creator_account_id: String,
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
    #[serde(rename = "id")]
    pub online_id: String,
    #[serde(rename = "len")]
    pub path_length: u32,
    #[serde(rename = "t")]
    pub created_time: i64,
    #[serde(skip)]
    pub data: Vec<u8>,
    #[serde(rename = "ver")]
    pub data_version: i32,
}
