mod baggages;
mod language;
mod qpid_areas;

#[cfg(feature = "ts")]
use typescript_type_def::TypeDef;

use std::collections::HashMap;

pub use baggages::*;
pub use language::*;
pub use qpid_areas::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(TypeDef))]
#[serde(rename_all = "camelCase")]
pub struct ObjectMetaData {
    pub uuid: String,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GameData {
    pub baggages: HashMap<u32, Baggage>,
    pub qpid_areas: HashMap<i32, QpidArea>,
    pub lost_baggages: HashMap<i32, Vec<u32>>,
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

    pub fn get_lost_baggages(&self, qpid_ids: &[i32]) -> HashMap<i32, Vec<&Baggage>> {
        let mut lost_baggages = HashMap::new();

        for qpid_id in qpid_ids {
            if let Some(baggage_ids) = self.lost_baggages.get(&qpid_id) {
                let mut baggages = Vec::with_capacity(baggage_ids.len());

                for baggage_id in baggage_ids {
                    if let Some(baggage) = self.baggages.get(baggage_id) {
                        baggages.push(baggage);
                    } else {
                        log::warn!("Baggage {} not found", baggage_id);
                    }
                }

                lost_baggages.insert(*qpid_id, baggages);
            }
        }

        lost_baggages
    }

    pub fn get_raw_materials_lost_baggages(&self) -> Vec<&Baggage> {
        let mut lost_baggages = Vec::new();

        for baggage in self.baggages.values() {
            if baggage.baggage_metadata.type_contents != ContentsType::RawMaterial {
                continue;
            }

            // Exclude any mission specific baggages that aren't usable by the player
            if baggage.baggage_metadata.mission_id != 0 {
                continue;
            }

            if let Some(name) = baggage.names.get(&Language::English) {
                // It seem that all the usable raw materials have a {0} in the name which is replaced by the amount
                if !name.contains("{0}") {
                    continue;
                }

                // todo: At the moment there's almost always two of every material added, but they are still different. Will have to check if they're both usable or not
            } else {
                continue;
            }

            lost_baggages.push(baggage);
        }

        lost_baggages
    }
}
