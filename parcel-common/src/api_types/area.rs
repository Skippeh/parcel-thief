use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum AreaHash {
    #[serde(rename = "5319")]
    EasternRegion = 5319,
    #[serde(rename = "22123")]
    CentralRegion = 22123,
    #[serde(rename = "21299")]
    WesternRegion = 21299,
}
