use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DevoteHighwayResourcesRequest {
    #[serde(rename = "ph")]
    pub put_histories: Vec<PutHistory>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PutHistory {
    #[serde(rename = "ci")]
    pub construction_id: i32,
    #[serde(rename = "n")]
    pub put_num: i32,
    #[serde(rename = "ri")]
    pub resource_id: i16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DevoteHighwayResourcesResponse;
