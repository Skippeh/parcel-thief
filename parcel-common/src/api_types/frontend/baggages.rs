use serde::Serialize;
use typescript_type_def::TypeDef;

#[derive(Debug, Clone, Serialize, TypeDef)]
#[serde(rename_all = "camelCase")]
pub struct BaggageListItem {
    pub name: String,
    pub amount: i32,
    pub category: String,
    pub location: String,
    pub creator: String,
}

#[derive(Debug, Clone, Serialize, TypeDef)]
#[serde(rename_all = "camelCase")]
pub struct ListSharedCargoResponse {
    pub baggages: Vec<BaggageListItem>,
}
