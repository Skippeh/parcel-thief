use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetStrandRequest {
    #[serde(rename = "add")]
    pub add_account_ids: Option<Vec<String>>,
    #[serde(rename = "del")]
    pub del_account_ids: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Default, Clone)]
pub struct SetStrandResponse;
