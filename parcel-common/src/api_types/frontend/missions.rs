#[cfg(feature = "ts")]
use typescript_type_def::TypeDef;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(TypeDef))]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum EditMissionData {
    #[serde(rename_all = "camelCase")]
    Delivery {
        start_qpid_id: i32,
        end_qpid_id: i32,
        baggage_amounts: Vec<BaggageAmount>,
        reward_amounts: Vec<BaggageAmount>,
    },
    #[serde(rename_all = "camelCase")]
    Collection {
        target_qpid_id: i32,
        baggage_amounts: Vec<BaggageAmount>,
        reward_amounts: Vec<BaggageAmount>,
    },
    #[serde(rename_all = "camelCase")]
    Recovery {
        target_qpid_id: i32,
        baggages: Vec<BaggageWithLocationAndAmount>,
        reward_amounts: Vec<BaggageAmount>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(TypeDef))]
#[serde(rename_all = "camelCase")]
pub struct BaggageAmount {
    pub name_hash: u32,
    pub amount: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(TypeDef))]
#[serde(rename_all = "camelCase")]
pub struct BaggageWithLocationAndAmount {
    pub name_hash: u32,
    pub amount: u32,
    pub location: (f32, f32, f32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(TypeDef))]
#[serde(rename_all = "camelCase")]
pub struct EditMissionRequest {
    pub mission_id: Option<String>,
    pub data: EditMissionData,
}
