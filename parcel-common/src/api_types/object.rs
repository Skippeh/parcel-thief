use serde::{Deserialize, Serialize};

use super::{area::AreaHash, mission::Mission, road::Road};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConstructionMaterials {
    #[serde(rename = "c")]
    pub contributor_account_id: String,
    /// The materials currently in this object
    #[serde(rename = "mat")]
    pub materials: [i32; 6],
    /// The materials to contribute
    #[serde(rename = "rmat")]
    pub materials_to_repair: [i32; 6],
    /// The time when these materials were contributed, expressed as epoch (milliseconds)
    #[serde(rename = "t")]
    pub contribute_time: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecycleMaterials {
    #[serde(rename = "c")]
    pub contributor_account_id: String,
    #[serde(rename = "mat")]
    pub materials: [i32; 6],
    #[serde(rename = "t")]
    pub time: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ObjectBaggage {
    #[serde(rename = "hs")]
    pub item_name_hash: i32,
    #[serde(rename = "mid")]
    pub mission_id: i32,
    #[serde(rename = "cr")]
    pub creator_account_id: String,
    #[serde(rename = "lf")]
    pub life: i32,
    #[serde(rename = "en")]
    pub endurance: i32,
    #[serde(rename = "hn")]
    pub handle: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Comment {
    #[serde(rename = "p")]
    pub phrases: Vec<i32>,
    #[serde(rename = "wr")]
    pub writer: String,
    #[serde(rename = "lp")]
    pub likes: i32,
    #[serde(rename = "pi")]
    pub parent_index: i8,
    #[serde(rename = "d")]
    pub is_deleted: bool,
    #[serde(rename = "r")]
    pub reference_object: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RopeInfo {
    #[serde(rename = "p")]
    pub pitch: i32,
    #[serde(rename = "h")]
    pub heading: i32,
    #[serde(rename = "l")]
    pub length: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoneInfo {
    #[serde(rename = "r")]
    pub resting_count: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BridgeInfo {
    #[serde(rename = "a")]
    pub angle: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ParkingInfo {
    #[serde(rename = "l")]
    pub location_id: i32,
    #[serde(rename = "dl")]
    pub dynamic_location_id: String,
    #[serde(rename = "cq")]
    pub current_qpid_id: i32,
    #[serde(rename = "pk")]
    pub is_parking: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VehicleInfo {
    #[serde(rename = "l")]
    pub location_id: i32,
    #[serde(rename = "dl")]
    pub dynamic_location_id: String,
    #[serde(rename = "cq")]
    pub current_qpid_id: i32,
    #[serde(rename = "pk")]
    pub is_parking: bool,
    #[serde(rename = "ls")]
    pub is_lost: bool,
    #[serde(rename = "rc")]
    pub is_race: bool,
    #[serde(rename = "ct")]
    pub customize_type: i32,
    #[serde(rename = "cc")]
    pub customize_color: i32,
    #[serde(rename = "nl")]
    pub new_position: (i32, i32, i32),
    #[serde(rename = "nr")]
    pub new_rotation: (i32, i32, i32),
    #[serde(rename = "exp")]
    pub exponent: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExtraInfo {
    #[serde(rename = "aq")]
    pub alternative_qpid_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomizeInfo {
    #[serde(rename = "cp")]
    pub customize_param: u32,
    #[serde(rename = "col")]
    pub customize_color: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Object {
    #[serde(rename = "c")]
    pub creator_account_id: String,
    #[serde(rename = "exp")]
    pub exponent: i32,
    #[serde(rename = "id")]
    pub object_id: String,
    #[serde(rename = "l")]
    pub position: (i32, i32, i32),
    #[serde(rename = "lp")]
    pub likes: u32,
    /// grid_x, grid_y, area_hash
    #[serde(rename = "m")]
    pub map_index: (i32, i32, AreaHash),
    #[serde(rename = "q")]
    pub qpid_id: i32,
    #[serde(rename = "r")]
    pub rotation: (i32, i32, i32),
    #[serde(rename = "st")]
    pub sub_type: String,
    #[serde(rename = "t")]
    pub object_type: String,
    #[serde(rename = "ut")]
    pub updated_time: i64,
    #[serde(rename = "mt")]
    pub construction_materials_contributions: Option<Vec<ConstructionMaterials>>,
    #[serde(rename = "rmt")]
    pub recycle_materials: Option<Vec<RecycleMaterials>>,
    #[serde(rename = "bgs")]
    pub baggages: Option<Vec<ObjectBaggage>>,
    #[serde(rename = "cm")]
    pub comments: Option<Vec<Comment>>,
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
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QpidObjectsResponse {
    #[serde(rename = "ro")]
    pub roads: Option<Vec<Road>>,
    #[serde(rename = "m")]
    pub missions: Option<Vec<Mission>>,
    #[serde(rename = "oa")]
    pub object_a: Option<Vec<Object>>,
    #[serde(rename = "ob")]
    pub object_b: Option<Vec<Object>>,
    #[serde(rename = "od")]
    pub object_d: Option<Vec<Object>>,
    #[serde(rename = "oe")]
    pub object_e: Option<Vec<Object>>,
    #[serde(rename = "op")]
    pub object_p: Option<Vec<Object>>,
}
