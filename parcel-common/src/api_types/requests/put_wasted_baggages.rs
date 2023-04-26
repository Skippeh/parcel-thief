use serde::{Deserialize, Serialize};

use super::get_wasted_baggages::WastedItem;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PutWastedBaggagesRequest {
    #[serde(rename = "qid")]
    pub qpid_id: i32,
    #[serde(rename = "wbs")]
    pub wasted_items: Vec<WastedItem>,
}

pub struct PutWastedBaggagesResponse;
