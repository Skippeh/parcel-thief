use serde::{Deserialize, Serialize};

use crate::api_types::object::Object;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetConstructionMaterialsRequest {
    #[serde(rename = "id")]
    pub object_id: String,
    #[serde(rename = "mat")]
    pub materials: [i32; 6],
    #[serde(rename = "rmat")]
    pub materials_to_repair: [i32; 6],
}

pub type SetConstructionMaterialsResponse = Object;
