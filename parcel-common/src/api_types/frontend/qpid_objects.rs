use serde::{Deserialize, Serialize};

#[cfg(feature = "ts")]
use typescript_type_def::TypeDef;

use crate::api_types::object::ObjectType;

use super::accounts::GameAccountSummary;

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "ts", derive(TypeDef))]
#[serde(rename_all = "camelCase")]
pub struct QpidObject {
    pub id: String,
    pub location: (f32, f32, f32),
    pub location_id: i32,
    pub object_type: QpidObjectType,
    pub unknown_type: Option<(String, String)>,
    /// Only applicable for vehicles. If true then the vehicle is not in a garage (i think).
    /// For all other object types this is always true.
    pub is_lost: bool,
    pub creator: GameAccountSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(TypeDef))]
#[serde(rename_all = "camelCase")]
pub enum QpidObjectType {
    Unknown,
    Ladder,
    ClimbingAnchor,
    Bridge,
    TimefallShelter,
    SafeHouse,
    Zipline,
    JumpRamp,
    ChiralBridge,
    Sign,
    PowerGenerator,
    Postbox,
    Watchtower,
    RestingStone,
    PeeMushroom,
    Motorbike,
    Truck,
    CargoCatapult,
    CargoCatapultPod,
}

impl<'a> From<(ObjectType, &'a str)> for QpidObjectType {
    fn from((ty, sub_type): (ObjectType, &'a str)) -> Self {
        match ty {
            ObjectType::Ladder => QpidObjectType::Ladder,
            ObjectType::PeeMushroom => QpidObjectType::PeeMushroom,
            ObjectType::Postbox => QpidObjectType::Postbox,
            ObjectType::PowerGenerator => QpidObjectType::PowerGenerator,
            ObjectType::RestingStone1 => QpidObjectType::RestingStone,
            ObjectType::RestingStone2 => QpidObjectType::RestingStone,
            ObjectType::Sign => QpidObjectType::Sign,
            ObjectType::WatchTower => QpidObjectType::Watchtower,
            ObjectType::SafeHouse => QpidObjectType::SafeHouse,
            ObjectType::Vehicle | ObjectType::LongRangeVehicle => match sub_type {
                "Motorbike" => QpidObjectType::Motorbike,
                "Truck" => QpidObjectType::Truck,
                _ => QpidObjectType::Unknown,
            },
            ObjectType::Bridge => QpidObjectType::Bridge,
            ObjectType::ClimbingAnchor => QpidObjectType::ClimbingAnchor,
            _ => QpidObjectType::Unknown,
        }
    }
}
