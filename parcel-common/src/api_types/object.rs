use diesel::{
    backend::Backend, deserialize::FromSql, serialize::ToSql, sql_types::Integer, AsExpression,
    FromSqlRow,
};
use serde::{Deserialize, Serialize};

use crate::serde_util::serialize_bool_to_number;

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
    pub recycle_time: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Baggage {
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
    #[serde(rename = "nl", skip_serializing_if = "Option::is_none")]
    pub new_position: Option<(i32, i32, i32)>,
    #[serde(rename = "nr", skip_serializing_if = "Option::is_none")]
    pub new_rotation: Option<(i32, i32, i32)>,
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

#[derive(
    Debug,
    Deserialize,
    Serialize,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    FromSqlRow,
    AsExpression,
)]
#[diesel(sql_type = Integer)]
#[repr(i32)]
pub enum ObjectType {
    #[serde(rename = "m")]
    M = 0,
    #[serde(rename = "z")]
    Z = 1,
    #[serde(rename = "c")]
    PowerGenerator = 2,
    #[serde(rename = "p")]
    Postbox = 3,
    #[serde(rename = "a")]
    A = 4,
    #[serde(rename = "r")]
    R1 = 5,
    #[serde(rename = "l")]
    Ladder = 6,
    #[serde(rename = "s")]
    S = 7,
    #[serde(rename = "w")]
    Watchtower = 8,
    #[serde(rename = "b")]
    B1 = 9,
    /// Subtype holds the type of the sign.
    #[serde(rename = "t")]
    Sign = 10,
    #[serde(rename = "v")]
    V = 11,
    #[serde(rename = "k")]
    K = 12,
    #[serde(rename = "n")]
    N = 13,
    #[serde(rename = "h")]
    H = 14,
    /// A stone created from a player sleeping.
    #[serde(rename = "e")]
    RestingStone = 15,
    #[serde(rename = "u")]
    U = 16,
    #[serde(rename = "i")]
    I = 17,
    #[serde(rename = "o")]
    O = 18,
    /// A mushroom created from a player peeing.
    #[serde(rename = "x")]
    PeeMushroom = 19,
    #[serde(rename = "B")]
    B2 = 20,
    #[serde(rename = "R")]
    R2 = 21,
    #[serde(rename = "S")]
    S2 = 22,
}

impl<DB> ToSql<Integer, DB> for ObjectType
where
    DB: Backend,
    i32: ToSql<Integer, DB>,
{
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, DB>,
    ) -> diesel::serialize::Result {
        match self {
            ObjectType::M => 0.to_sql(out),
            ObjectType::Z => 1.to_sql(out),
            ObjectType::PowerGenerator => 2.to_sql(out),
            ObjectType::Postbox => 3.to_sql(out),
            ObjectType::A => 4.to_sql(out),
            ObjectType::R1 => 5.to_sql(out),
            ObjectType::Ladder => 6.to_sql(out),
            ObjectType::S => 7.to_sql(out),
            ObjectType::Watchtower => 8.to_sql(out),
            ObjectType::B1 => 9.to_sql(out),
            ObjectType::Sign => 10.to_sql(out),
            ObjectType::V => 11.to_sql(out),
            ObjectType::K => 12.to_sql(out),
            ObjectType::N => 13.to_sql(out),
            ObjectType::H => 14.to_sql(out),
            ObjectType::RestingStone => 15.to_sql(out),
            ObjectType::U => 16.to_sql(out),
            ObjectType::I => 17.to_sql(out),
            ObjectType::O => 18.to_sql(out),
            ObjectType::PeeMushroom => 19.to_sql(out),
            ObjectType::B2 => 20.to_sql(out),
            ObjectType::R2 => 21.to_sql(out),
            ObjectType::S2 => 22.to_sql(out),
        }
    }
}

impl<DB> FromSql<Integer, DB> for ObjectType
where
    DB: Backend,
    i32: FromSql<Integer, DB>,
{
    fn from_sql(bytes: diesel::backend::RawValue<'_, DB>) -> diesel::deserialize::Result<Self> {
        match i32::from_sql(bytes)? {
            0 => Ok(ObjectType::M),
            1 => Ok(ObjectType::Z),
            2 => Ok(ObjectType::PowerGenerator),
            3 => Ok(ObjectType::Postbox),
            4 => Ok(ObjectType::A),
            5 => Ok(ObjectType::R1),
            6 => Ok(ObjectType::Ladder),
            7 => Ok(ObjectType::S),
            8 => Ok(ObjectType::Watchtower),
            9 => Ok(ObjectType::B1),
            10 => Ok(ObjectType::Sign),
            11 => Ok(ObjectType::V),
            12 => Ok(ObjectType::K),
            13 => Ok(ObjectType::N),
            14 => Ok(ObjectType::H),
            15 => Ok(ObjectType::RestingStone),
            16 => Ok(ObjectType::U),
            17 => Ok(ObjectType::I),
            18 => Ok(ObjectType::O),
            19 => Ok(ObjectType::PeeMushroom),
            20 => Ok(ObjectType::B2),
            21 => Ok(ObjectType::R2),
            22 => Ok(ObjectType::S2),
            other => Err(format!("Unknown ObjectType variant: {}", other).into()),
        }
    }
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
    pub object_type: ObjectType,
    #[serde(rename = "ut")]
    pub updated_time: i64,
    #[serde(rename = "mt", skip_serializing_if = "Option::is_none")]
    pub construction_materials_contributions: Option<Vec<ConstructionMaterials>>,
    #[serde(rename = "rmt", skip_serializing_if = "Option::is_none")]
    pub recycle_materials: Option<Vec<RecycleMaterials>>,
    #[serde(rename = "bgs", skip_serializing_if = "Option::is_none")]
    pub baggages: Option<Vec<Baggage>>,
    #[serde(rename = "cm", skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<Comment>>,
    #[serde(rename = "ri", skip_serializing_if = "Option::is_none")]
    pub rope_info: Option<RopeInfo>,
    #[serde(rename = "si", skip_serializing_if = "Option::is_none")]
    pub stone_info: Option<StoneInfo>,
    #[serde(rename = "bi", skip_serializing_if = "Option::is_none")]
    pub bridge_info: Option<BridgeInfo>,
    #[serde(rename = "pi", skip_serializing_if = "Option::is_none")]
    pub parking_info: Option<ParkingInfo>,
    #[serde(rename = "vi", skip_serializing_if = "Option::is_none")]
    pub vehicle_info: Option<VehicleInfo>,
    #[serde(rename = "ei", skip_serializing_if = "Option::is_none")]
    pub extra_info: Option<ExtraInfo>,
    #[serde(rename = "ci", skip_serializing_if = "Option::is_none")]
    pub customize_info: Option<CustomizeInfo>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(
        rename = "p",
        serialize_with = "serialize_bool_to_number",
        skip_deserializing
    )]
    pub priority: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct QpidObjectsResponse {
    #[serde(rename = "ro", skip_serializing_if = "Option::is_none")]
    pub roads: Option<Vec<Road>>,
    #[serde(rename = "m", skip_serializing_if = "Option::is_none")]
    pub missions: Option<Vec<Mission>>,
    #[serde(rename = "oa", skip_serializing_if = "Option::is_none")]
    pub object_a: Option<Vec<Object>>,
    #[serde(rename = "ob", skip_serializing_if = "Option::is_none")]
    pub object_b: Option<Vec<Object>>,
    #[serde(rename = "od", skip_serializing_if = "Option::is_none")]
    pub object_d: Option<Vec<Object>>,
    #[serde(rename = "oe", skip_serializing_if = "Option::is_none")]
    pub object_e: Option<Vec<Object>>,
    #[serde(rename = "op", skip_serializing_if = "Option::is_none")]
    pub object_p: Option<Vec<Object>>,
}
