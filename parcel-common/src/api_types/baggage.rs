use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ObjectBaggage {
    #[serde(rename = "hs")]
    pub item_name_hash: i32,
    #[serde(rename = "mid")]
    pub mission_id: i32,
    #[serde(rename = "cr")]
    pub creator_account_id: String,
    #[serde(rename = "lf")]
    pub life: i32,
    #[serde(rename = "en")]
    pub endurance: i32,
    #[serde(rename = "hn")]
    pub handle: i32,
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
