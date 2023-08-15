#[cfg(feature = "diesel")]
use std::io::Write;

#[cfg(feature = "diesel")]
use diesel::{
    backend::Backend,
    deserialize::FromSql,
    pg::Pg,
    serialize::{IsNull, ToSql},
    sql_types::Text,
    AsExpression, FromSqlRow,
};
use serde::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

use crate::serde_util::serialize_bool_to_number;

use super::{area::AreaHash, mission::Mission, road::Road};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConstructionMaterials {
    /// The account id of the user that has contributed these materials
    #[serde(rename = "c")]
    pub contributor_account_id: String,
    /// The total amount of materials contributed for upgrades
    #[serde(rename = "mat")]
    pub materials: [i32; 6],
    /// The total amount of materials contributed for repairs
    #[serde(rename = "rmat")]
    pub materials_to_repair: [i32; 6],
    /// The most recent time when these materials were contributed, expressed as epoch (milliseconds)
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

#[derive(Debug, Deserialize_enum_str, Serialize_enum_str, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "diesel", derive(FromSqlRow, AsExpression))]
#[cfg_attr(feature = "diesel", diesel(sql_type = Text))]
pub enum ObjectType {
    #[serde(rename = "m")]
    M,
    #[serde(rename = "z")]
    Z,
    #[serde(rename = "c")]
    PowerGenerator,
    #[serde(rename = "p")]
    Postbox,
    #[serde(rename = "a")]
    A,
    #[serde(rename = "r")]
    ClimbingAnchor,
    #[serde(rename = "l")]
    Ladder,
    #[serde(rename = "s")]
    SafeHouse,
    #[serde(rename = "w")]
    WatchTower,
    #[serde(rename = "b")]
    Bridge,
    /// Subtype holds the type of the sign.
    #[serde(rename = "t")]
    Sign,
    #[serde(rename = "v")]
    Vehicle,
    #[serde(rename = "k")]
    K,
    /// A stone created from a player sleeping. First of two types (not sure what the difference is)
    #[serde(rename = "n")]
    RestingStone1,
    #[serde(rename = "h")]
    H,
    /// The second type of stone created from a player sleeping (not sure what the difference is)
    #[serde(rename = "e")]
    RestingStone2,
    #[serde(rename = "u")]
    U,
    #[serde(rename = "i")]
    I,
    #[serde(rename = "o")]
    O,
    /// A mushroom created from a player peeing.
    #[serde(rename = "x")]
    PeeMushroom,
    #[serde(rename = "B")]
    B2,
    #[serde(rename = "R")]
    R2,
    #[serde(rename = "S")]
    S2,

    #[serde(other)]
    Unknown(String),
}

#[cfg(feature = "diesel")]
impl ToSql<Text, Pg> for ObjectType {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Pg>,
    ) -> diesel::serialize::Result {
        let val_str = serde_json::to_string(self)?;

        out.write_all(val_str.trim_matches('"').as_bytes())?;
        Ok(IsNull::No)
    }
}

#[cfg(feature = "diesel")]
impl FromSql<Text, Pg> for ObjectType
where
    String: FromSql<Text, Pg>,
{
    fn from_sql(bytes: <Pg as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let val_str = String::from_utf8(bytes.as_bytes().to_vec())?;

        Ok(serde_json::from_str(&format!("\"{}\"", &val_str))?)
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
    pub recycle_materials_contributions: Option<Vec<RecycleMaterials>>,
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
