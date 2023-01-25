use serde::{Deserialize, Serialize};

use crate::api_types::object::QpidObjectsResponse;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetQpidObjectsRequest {
    #[serde(rename = "ids")]
    pub qpid_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetQpidObjectsResponse {
    #[serde(rename = "n")]
    pub normal: QpidObjectsResponse,
}
