use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct DevoteHighwayResourcesRequest {
    #[serde(rename = "ph")]
    pub put_histories: Vec<PutHistory>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct PutHistory {
    #[serde(rename = "ci")]
    pub construction_id: i32,
    #[serde(rename = "n")]
    #[validate(range(min = 1))]
    pub put_num: i32,
    #[serde(rename = "ri")]
    pub resource_id: i16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DevoteHighwayResourcesResponse;
