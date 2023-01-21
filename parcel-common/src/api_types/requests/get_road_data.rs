use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetRoadDataRequest {
    /// The id of the road's object.
    ///
    /// Note that road in this context refers to the player generated paths and not highway segments.
    #[serde(rename = "id")]
    pub object_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetRoadDataResponse {
    /// Base64 string of some binary data (unknown format)
    #[serde(rename = "data")]
    pub data: String,
}
