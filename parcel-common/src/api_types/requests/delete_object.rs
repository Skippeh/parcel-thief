use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteObjectRequest {
    #[serde(rename = "id")]
    pub object_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteObjectResponse;
