use serde::{Deserialize, Serialize};

use crate::api_types::baggage::WastedBaggage;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetWastedBaggagesRequest {
    #[serde(rename = "qpids")]
    pub qpid_ids: Vec<QpidIds>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QpidIds {
    #[serde(rename = "id")]
    pub qpid_id: i32,
    #[serde(rename = "lgd")]
    pub last_login_time: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetWastedBaggagesResponse {
    #[serde(rename = "ud")]
    pub update_date: i64,
    #[serde(rename = "ws")]
    pub baggages: Vec<WastedBaggage>,
}
