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

impl GameData {
    pub fn baggage_name(&self, name_hash: u32, language: Language) -> Option<&String> {
        self.baggages
            .get(&name_hash)
            .map(|b| b.names.get(&language))
            .flatten()
    }

    pub fn qpid_area_name(&self, qpid_id: i32, language: Language) -> Option<&String> {
        self.qpid_areas
            .get(&qpid_id)
            .map(|a| a.names.get(&language))
            .flatten()
    }
}
