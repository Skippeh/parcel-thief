use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetHighwayResourcesRequest {
    #[serde(rename = "cds")]
    pub constructions: Option<Vec<ConstructionRequest>>,
    #[serde(rename = "rids")]
    pub resource_ids: Vec<i16>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConstructionRequest {
    #[serde(rename = "cid")]
    pub construction_id: i32,
    /// This date is expressed as microseconds since 0001-01-01 00:00:00
    #[serde(rename = "lgd")]
    pub last_login_date: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetHighwayResourcesResponse {
    #[serde(rename = "cs")]
    pub construction_contributors: Vec<ConstructionContributors>,
    #[serde(rename = "pr")]
    pub put_resources: Vec<PutResource>,
    #[serde(rename = "ul")]
    pub users_like: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConstructionContributors {
    #[serde(rename = "cid")]
    pub construction_id: i32,
    #[serde(rename = "cs")]
    pub contributors: Vec<Contributor>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Contributor {
    #[serde(rename = "l")]
    pub likes: i64,
    #[serde(rename = "u")]
    pub account_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PutResource {
    #[serde(rename = "cid")]
    pub construction_id: i32,
    #[serde(rename = "n")]
    pub put_num: i64,
    #[serde(rename = "rid")]
    pub resource_id: i16,
    /// Seems to always be 0
    #[serde(rename = "un")]
    pub users_put_num: i64,
}
