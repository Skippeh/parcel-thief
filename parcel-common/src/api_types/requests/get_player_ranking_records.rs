use serde::{Deserialize, Serialize};

use crate::api_types::rank::{RankingRecord, RankingRewards};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetPlayerRankingRecordsRequest {
    #[serde(rename = "sid")]
    pub season_id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetPlayerRankingRecordsResponse {
    #[serde(rename = "rs")]
    pub records: Vec<RankingRecord>,
    #[serde(rename = "rw")]
    pub rewards: RankingRewards,
    #[serde(rename = "ut")]
    pub update_time: i64,
}
