use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteWastedBaggagesRequest {
    #[serde(rename = "reqs")]
    pub delete_requests: Vec<DeleteRequest>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteRequest {
    #[serde(rename = "bid")]
    pub baggage_id: String,
    #[serde(rename = "uid")]
    pub account_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteWastedBaggagesResponse;
