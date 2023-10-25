use std::collections::HashMap;

use parcel_game_data::{BaggageMetaData, ContentsType, Language, ObjectMetaData};
use serde::Serialize;

#[cfg(feature = "ts")]
use typescript_type_def::TypeDef;

use super::accounts::GameAccountSummary;

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "ts", derive(TypeDef))]
#[serde(rename_all = "camelCase")]
pub struct SharedCargoListItem {
    pub name: String,
    pub amount: i32,
    pub category: String,
    pub location: String,
    pub creator: String,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "ts", derive(TypeDef))]
#[serde(rename_all = "camelCase")]
pub struct LostCargoListItem {
    pub name: String,
    pub amount: i32,
    pub category: String,
    pub location: String,
    pub end_location: String,
    pub creator: String,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "ts", derive(TypeDef))]
#[serde(rename_all = "camelCase")]
pub struct WastedCargoListItem {
    pub name: String,
    pub category: String,
    pub broken: bool,
    pub location: String,
    pub creator: String,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "ts", derive(TypeDef))]
#[serde(rename_all = "camelCase")]
pub struct Baggage {
    pub mission_id: String,
    pub id: i64,
    pub name: String,
    pub amount: i32,
    pub location: (f32, f32, f32),
    pub location_id: i32,
    pub target_location_id: Option<i32>,
    pub target_location_name: Option<String>,
    pub category: ContentsType,
    pub is_wasted: bool,
    pub is_broken: bool,
    pub creator: GameAccountSummary,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "ts", derive(TypeDef))]
#[serde(rename_all = "camelCase")]
pub struct ListSharedCargoResponse {
    pub baggages: Vec<SharedCargoListItem>,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "ts", derive(TypeDef))]
#[serde(rename_all = "camelCase")]
pub struct ListLostCargoResponse {
    pub baggages: Vec<LostCargoListItem>,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "ts", derive(TypeDef))]
#[serde(rename_all = "camelCase")]
pub struct ListWastedCargoResponse {
    pub baggages: Vec<WastedCargoListItem>,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "ts", derive(TypeDef))]
#[serde(rename_all = "camelCase")]
pub struct ListLostBaggagesResponse {
    pub qpid_baggages: HashMap<i32, Vec<LocalizedBaggageData>>,
    pub raw_material_baggages: Vec<LocalizedBaggageData>,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "ts", derive(TypeDef))]
#[serde(rename_all = "camelCase")]
pub struct LocalizedBaggageData {
    pub name_hash: u32,
    pub object_metadata: ObjectMetaData,
    pub baggage_metadata: BaggageMetaData,
    pub name: String,
    pub description: String,
}

impl LocalizedBaggageData {
    pub fn from_baggage_data(
        baggage: parcel_game_data::Baggage,
        language: Language,
    ) -> LocalizedBaggageData {
        let name = baggage
            .names
            .get(&language)
            .cloned()
            .unwrap_or_default()
            .replace("{0}", &baggage.baggage_metadata.amount.to_string());

        LocalizedBaggageData {
            name_hash: baggage.name_hash,
            object_metadata: baggage.object_metadata,
            baggage_metadata: baggage.baggage_metadata,
            name,
            description: baggage
                .descriptions
                .get(&language)
                .cloned()
                .unwrap_or_default(),
        }
    }
}
