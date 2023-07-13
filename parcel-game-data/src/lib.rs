mod baggages;
mod language;
mod qpid_areas;

pub use baggages::*;
pub use language::*;
pub use qpid_areas::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectMetaData {
    pub uuid: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GameData {
    pub baggages: Vec<Baggage>,
    pub qpid_areas: Vec<QpidArea>,
}
