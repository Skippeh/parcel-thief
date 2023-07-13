mod baggages;
mod language;
mod qpid_areas;

use std::collections::HashMap;

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
    pub baggages: HashMap<u32, Baggage>,
    pub qpid_areas: HashMap<i32, QpidArea>,
}
