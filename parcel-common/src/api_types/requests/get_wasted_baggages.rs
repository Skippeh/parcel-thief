use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WastedBaggage {
    #[serde(rename = "bid")]
    pub baggage_id: String,
    #[serde(rename = "uid")]
    pub account_id: String,
    #[serde(rename = "qid")]
    pub qpid_id: i32,
    #[serde(rename = "wb")]
    pub item: WastedItem,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WastedItem {
    #[serde(rename = "b")]
    pub broken: bool,
    #[serde(rename = "h")]
    pub item_hash: i32,
    #[serde(rename = "x")]
    pub x: i32,
    #[serde(rename = "y")]
    pub y: i32,
    #[serde(rename = "z")]
    pub z: i32,
}
