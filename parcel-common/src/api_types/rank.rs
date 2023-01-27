use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RankingRecord {
    #[serde(rename = "c")]
    pub category: i32,
    #[serde(rename = "cc")]
    pub clear_count: u32,
    #[serde(rename = "tc")]
    pub try_count: u32,
    #[serde(rename = "d")]
    pub difficulty: i32,
    #[serde(rename = "dr")]
    pub detail_rank: i32,
    #[serde(rename = "f")]
    pub flags: i32,
    #[serde(rename = "mid")]
    pub mission_id: i32,
    #[serde(rename = "r")]
    pub rank: i32,
    #[serde(rename = "s")]
    pub score: i32,
    #[serde(rename = "sid")]
    pub season_id: i32,
}
