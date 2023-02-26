use serde::{Deserialize, Serialize};

use crate::api_types::{
    area::AreaHash,
    object::{
        BridgeInfo, Comment, CustomizeInfo, ExtraInfo, Object, ObjectType, ParkingInfo, RopeInfo,
        StoneInfo, VehicleInfo,
    },
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateObjectRequest {
    #[serde(rename = "exp")]
    pub exponent: i32,
    #[serde(rename = "lp")]
    pub likes: u32,
    #[serde(rename = "m")]
    pub area_hash: AreaHash,
    #[serde(rename = "p")]
    pub priority: i32,
    #[serde(rename = "px")]
    pub pos_x: i32,
    #[serde(rename = "py")]
    pub pos_y: i32,
    #[serde(rename = "pz")]
    pub pos_z: i32,
    #[serde(rename = "rx")]
    pub rot_x: i32,
    #[serde(rename = "ry")]
    pub rot_y: i32,
    #[serde(rename = "rz")]
    pub rot_z: i32,
    #[serde(rename = "x")]
    pub grid_x: i32,
    #[serde(rename = "y")]
    pub grid_y: i32,
    #[serde(rename = "q")]
    pub qpid_id: i32,
    #[serde(rename = "st")]
    pub sub_type: String,
    #[serde(rename = "t")]
    pub object_type: ObjectType,
    #[serde(rename = "c")]
    pub comment: Option<Comment>,
    #[serde(rename = "ri")]
    pub rope_info: Option<RopeInfo>,
    #[serde(rename = "si")]
    pub stone_info: Option<StoneInfo>,
    #[serde(rename = "bi")]
    pub bridge_info: Option<BridgeInfo>,
    #[serde(rename = "pi")]
    pub parking_info: Option<ParkingInfo>,
    #[serde(rename = "vi")]
    pub vehicle_info: Option<VehicleInfo>,
    #[serde(rename = "ei")]
    pub extra_info: Option<ExtraInfo>,
    #[serde(rename = "ci")]
    pub customize_info: Option<CustomizeInfo>,
}

pub type CreateObjectResponse = Object;
