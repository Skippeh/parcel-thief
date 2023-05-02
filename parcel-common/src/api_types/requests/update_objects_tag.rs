use serde::{Deserialize, Serialize};

use crate::api_types::object::Object;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateObjectsTagRequest {
    #[serde(rename = "add")]
    pub add: Option<Vec<String>>,
    #[serde(rename = "del")]
    pub delete: Option<Vec<String>>,
    pub tag: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateObjectsTagResponse {
    #[serde(rename = "objects")]
    pub objects: Vec<Object>,
}
