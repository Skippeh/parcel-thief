use serde::Serialize;

#[cfg(feature = "ts")]
use typescript_type_def::TypeDef;

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
