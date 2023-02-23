use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BasicPlayerProfile {
    #[serde(rename = "bid")]
    pub banner_id: i32,
    #[serde(rename = "bmd")]
    pub num_bronze_medals: i32,
    #[serde(rename = "bmdl")]
    pub num_bronze_medals_large: i32,
    #[serde(rename = "db")]
    pub delivered_baggage: i32,
    #[serde(rename = "dr")]
    pub delivery_rank: i32,
    #[serde(rename = "dw")]
    pub delivered_weight: i32,
    #[serde(rename = "ebr")]
    pub evaluation_bridge: i32,
    #[serde(rename = "edl")]
    pub evaluation_delivery: i32,
    #[serde(rename = "esf")]
    pub evaluation_safety: i32,
    #[serde(rename = "esp")]
    pub evaluation_speed: i32,
    #[serde(rename = "esv")]
    pub evaluation_service: i32,
    #[serde(rename = "gmd")]
    pub num_gold_medals: i32,
    #[serde(rename = "gmdl")]
    pub num_gold_medals_large: i32,
    /// legend stars
    #[serde(rename = "lc")]
    pub legend_count: i32,
    /// Last login date expressed in epoch (milliseconds)
    #[serde(rename = "ll")]
    pub last_login: i64,
    #[serde(rename = "md")]
    pub distance_traveled: i32,
    /// Maybe a bitflag of unlocked music tracks?
    #[serde(rename = "mot")]
    pub music_open_tracks: u64,
    /// Seems to always be 0
    #[serde(rename = "nm")]
    pub name_hash: i32,
    #[serde(rename = "pmd")]
    pub num_platinum_medals: i32,
    #[serde(rename = "pmdl")]
    pub num_platinum_medals_large: i32,
    #[serde(rename = "rln")]
    pub num_likes_received_npc: i32,
    #[serde(rename = "rlo")]
    pub num_likes_received_online: i32,
    #[serde(rename = "rmd")]
    pub num_rainbow_medals: i32,
    #[serde(rename = "rmdl")]
    pub num_rainbow_medals_large: i32,
    /// legend of legends stars
    #[serde(rename = "slc")]
    pub super_legend_count: i32,
    #[serde(rename = "smd")]
    pub num_silver_medals: i32,
    #[serde(rename = "smdl")]
    pub num_silver_medals_large: i32,
    /// legend of legends of legends stars
    #[serde(rename = "sslc")]
    pub ss_legend_count: i32,
}
