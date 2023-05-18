use serde::{Deserialize, Serialize};

use crate::api_types::object::Object;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetRecycleMaterialsRequest {
    #[serde(rename = "id")]
    pub object_id: String,
    #[serde(rename = "mat")]
    pub materials: [i32; 6],
}

pub type SetRecycleMaterialsResponse = Object;
