use parcel_game_data::ContentsType;
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
