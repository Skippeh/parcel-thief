use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerProfile {
    pub bid: i32,
    pub bmd: i32,
    pub bmdl: i32,
    #[serde(rename = "db")]
    pub delivered_cargo: i32,
    pub dr: i32,
    #[serde(rename = "dw")]
    pub delivered_weight: i32,
    pub ebr: i32,
    pub edl: i32,
    pub sef: i32,
    pub esp: i32,
    pub esv: i32,
    pub gmd: i32,
    pub gmdl: i32,
    /// legend stars
    #[serde(rename = "lc")]
    pub legend_stars: i32,
    /// Last login date expressed in epoch (milliseconds)
    #[serde(rename = "ll")]
    pub last_login: u64,
    #[serde(rename = "md")]
    pub distance_traveled: i32,
    pub mot: i32,
    pub nm: i32,
    pub pmd: i32,
    pub pmdl: i32,
    #[serde(rename = "rln")]
    pub received_likes_npc: i32,
    #[serde(rename = "rlo")]
    pub received_likes_online: i32,
    pub rmd: i32,
    pub rmdl: i32,
    /// legend of legends stars
    #[serde(rename = "slc")]
    pub super_legend_stars: i32,
    pub smd: i32,
    pub smdl: i32,
    /// legend of legends of legends stars
    #[serde(rename = "sslc")]
    pub super_super_legend_stars: i32,
}
