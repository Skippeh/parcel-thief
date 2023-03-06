use serde::{Deserialize, Serialize};

use crate::api_types::object::{CustomizeInfo, ExtraInfo, ParkingInfo, StoneInfo, VehicleInfo};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateObjectRequest {
    #[serde(rename = "id")]
    pub object_id: String,
    #[serde(rename = "si")]
    pub stone_info: Option<StoneInfo>,
    #[serde(rename = "pi")]
    pub parking_info: Option<ParkingInfo>,
    #[serde(rename = "vi")]
    pub vehicle_info: Option<VehicleInfo>,
    #[serde(rename = "ci")]
    pub customize_info: Option<CustomizeInfo>,
    #[serde(rename = "ei")]
    pub extra_info: Option<ExtraInfo>,
    // bridge_info and comment is intentionally excluded
}

pub struct UpdateObjectResponse;
