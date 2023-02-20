use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(
    Debug, Serialize_repr, Deserialize_repr, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash,
)]
#[repr(u32)]
pub enum AreaHash {
    #[serde(rename = "5319")]
    EasternRegion = 5319,
    #[serde(rename = "22123")]
    CentralRegion = 22123,
    #[serde(rename = "21299")]
    WesternRegion = 21299,
}
