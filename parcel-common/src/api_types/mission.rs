use diesel::{
    backend::Backend, deserialize::FromSql, serialize::ToSql, sql_types::Integer, AsExpression,
    FromSqlRow,
};
use serde::{Deserialize, Serialize};

use crate::serde_util::deserialize_bool_from_number;

use super::area::AreaHash;

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Copy,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    Hash,
    FromSqlRow,
    AsExpression,
)]
#[diesel(sql_type = Integer)]
#[repr(i32)]
pub enum OnlineMissionType {
    #[serde(rename = "Unknown_online_type")]
    UnknownOnlineType = 0,
    #[serde(rename = "Online_supply")]
    OnlineSupply = 1,
    #[serde(rename = "Private")]
    Private = 2,
    #[serde(rename = "Dynamic")]
    Dynamic = 3,
    #[serde(rename = "Static")]
    Static = 4,
    #[serde(rename = "Shared_last_stranding")]
    SharedLastStranding = 5,
}

impl<DB> ToSql<Integer, DB> for OnlineMissionType
where
    DB: Backend,
    i32: ToSql<Integer, DB>,
{
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, DB>,
    ) -> diesel::serialize::Result {
        match self {
            Self::UnknownOnlineType => 0.to_sql(out),
            Self::OnlineSupply => 1.to_sql(out),
            Self::Private => 2.to_sql(out),
            Self::Dynamic => 3.to_sql(out),
            Self::Static => 4.to_sql(out),
            Self::SharedLastStranding => 5.to_sql(out),
        }
    }
}

impl<DB> FromSql<Integer, DB> for OnlineMissionType
where
    DB: Backend,
    i32: FromSql<Integer, DB>,
{
    fn from_sql(bytes: diesel::backend::RawValue<'_, DB>) -> diesel::deserialize::Result<Self> {
        match i32::from_sql(bytes)? {
            0 => Ok(Self::UnknownOnlineType),
            1 => Ok(Self::OnlineSupply),
            2 => Ok(Self::Private),
            3 => Ok(Self::Dynamic),
            4 => Ok(Self::Static),
            5 => Ok(Self::SharedLastStranding),
            other => Err(format!("Unknown OnlineMissionType variant: {}", other).into()),
        }
    }
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Copy,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    Hash,
    FromSqlRow,
    AsExpression,
)]
#[diesel(sql_type = Integer)]
#[repr(i32)]
pub enum MissionType {
    #[serde(rename = "Delivery")]
    Delivery = 0,
    #[serde(rename = "Collect")]
    Collect = 1,
    #[serde(rename = "LostObject")]
    LostObject = 2,
    #[serde(rename = "Supply")]
    Supply = 3,
    #[serde(rename = "Special")]
    Special = 4,
    #[serde(rename = "Free")]
    Free = 5,
}

impl<DB> ToSql<Integer, DB> for MissionType
where
    DB: Backend,
    i32: ToSql<Integer, DB>,
{
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, DB>,
    ) -> diesel::serialize::Result {
        match self {
            Self::Delivery => 0.to_sql(out),
            Self::Collect => 1.to_sql(out),
            Self::LostObject => 2.to_sql(out),
            Self::Supply => 3.to_sql(out),
            Self::Special => 4.to_sql(out),
            Self::Free => 5.to_sql(out),
        }
    }
}

impl<DB> FromSql<Integer, DB> for MissionType
where
    DB: Backend,
    i32: FromSql<Integer, DB>,
{
    fn from_sql(bytes: diesel::backend::RawValue<'_, DB>) -> diesel::deserialize::Result<Self> {
        match i32::from_sql(bytes)? {
            0 => Ok(Self::Delivery),
            1 => Ok(Self::Collect),
            2 => Ok(Self::LostObject),
            3 => Ok(Self::Supply),
            4 => Ok(Self::Special),
            5 => Ok(Self::Free),
            other => Err(format!("Unknown MissionType variant: {}", other).into()),
        }
    }
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Copy,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    Hash,
    FromSqlRow,
    AsExpression,
)]
#[diesel(sql_type = Integer)]
#[repr(i32)]
pub enum ProgressState {
    #[serde(rename = "Invalid")]
    Invalid = 0,
    #[serde(rename = "Available")]
    Available = 1,
    #[serde(rename = "Ready")]
    Ready = 2,
    #[serde(rename = "Progress")]
    Progress = 4,
    #[serde(rename = "Failed")]
    Failed = 8,
    #[serde(rename = "Success")]
    Success = 16,
    #[serde(rename = "Cancel")]
    Cancel = 32,
    #[serde(rename = "Not_available")]
    NotAvailable = 64,
    #[serde(rename = "Returned")]
    Returned = 128,
    #[serde(rename = "Used")]
    Used = 256,
    #[serde(rename = "Missing")]
    Missing = 512,
    #[serde(rename = "Consign")]
    Consign = 1024,
    #[serde(rename = "Complete_automation")]
    CompleteAutomation = 2048,
}

impl<DB> ToSql<Integer, DB> for ProgressState
where
    DB: Backend,
    i32: ToSql<Integer, DB>,
{
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, DB>,
    ) -> diesel::serialize::Result {
        match self {
            Self::Invalid => 0.to_sql(out),
            Self::Available => 1.to_sql(out),
            Self::Ready => 2.to_sql(out),
            Self::Progress => 3.to_sql(out),
            Self::Failed => 4.to_sql(out),
            Self::Success => 5.to_sql(out),
            Self::Cancel => 6.to_sql(out),
            Self::NotAvailable => 7.to_sql(out),
            Self::Returned => 8.to_sql(out),
            Self::Used => 9.to_sql(out),
            Self::Missing => 10.to_sql(out),
            Self::Consign => 11.to_sql(out),
            Self::CompleteAutomation => 12.to_sql(out),
        }
    }
}

impl<DB> FromSql<Integer, DB> for ProgressState
where
    DB: Backend,
    i32: FromSql<Integer, DB>,
{
    fn from_sql(bytes: diesel::backend::RawValue<'_, DB>) -> diesel::deserialize::Result<Self> {
        match i32::from_sql(bytes)? {
            0 => Ok(Self::Invalid),
            1 => Ok(Self::Available),
            2 => Ok(Self::Ready),
            3 => Ok(Self::Progress),
            4 => Ok(Self::Failed),
            5 => Ok(Self::Success),
            6 => Ok(Self::Cancel),
            7 => Ok(Self::NotAvailable),
            8 => Ok(Self::Returned),
            9 => Ok(Self::Used),
            10 => Ok(Self::Missing),
            11 => Ok(Self::Consign),
            12 => Ok(Self::CompleteAutomation),
            other => Err(format!("Unknown ProgressState variant: {}", other).into()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct SupplyInfo {
    #[serde(rename = "h")]
    pub item_hash: i64,
    #[serde(rename = "n")]
    pub amount: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct DynamicMissionInfo {
    #[serde(rename = "ch")]
    pub client_name_hash: i32,
    #[serde(rename = "rh")]
    pub reward_name_hash: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct DynamicLocationInfo {
    #[serde(rename = "id")]
    pub location_id: String,
    #[serde(rename = "x")]
    pub x: i32,
    #[serde(rename = "y")]
    pub y: i32,
    #[serde(rename = "z")]
    pub z: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct AmmoInfo {
    #[serde(rename = "aid")]
    pub ammo_id: String,
    #[serde(rename = "cc")]
    pub clip_count: i16,
    #[serde(rename = "c")]
    pub count: i16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Baggage {
    #[serde(rename = "a")]
    pub amount: i32,
    #[serde(rename = "nm")]
    pub name_hash: i32,
    #[serde(rename = "ui")]
    pub user_index: i32,
    #[serde(rename = "x")]
    pub x: i32,
    #[serde(rename = "y")]
    pub y: i32,
    #[serde(rename = "z")]
    pub z: i32,
    #[serde(
        rename = "ret",
        skip_serializing,
        deserialize_with = "deserialize_bool_from_number"
    )]
    pub is_returned: bool,
    #[serde(rename = "am")]
    pub ammo_info: Option<AmmoInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Mission {
    #[serde(rename = "aid")]
    pub area_hash: AreaHash,
    #[serde(rename = "cr")]
    pub creator_account_id: String,
    /// The account id of the player that currently "owns" this mission
    #[serde(skip)]
    pub worker_account_id: Option<String>,
    #[serde(rename = "qid")]
    pub qpid_id: i32,
    #[serde(rename = "sid")]
    pub qpid_start_location: i32,
    #[serde(rename = "eid")]
    pub qpid_end_location: i32,
    #[serde(rename = "did")]
    pub qpid_delivered_location: i32,
    /// The id of this mission, generated by the server
    #[serde(rename = "id")]
    pub online_id: String,
    /// The id of the game mission, provided by the client, 0 is none
    #[serde(rename = "mid")]
    pub mission_static_id: i64,
    #[serde(rename = "mt")]
    pub mission_type: MissionType,
    #[serde(rename = "omt")]
    pub online_mission_type: OnlineMissionType,
    #[serde(rename = "pr")]
    pub progress_state: ProgressState,
    /// The history of account id's of players who been involved in this mission
    #[serde(rename = "rels")]
    pub relations: Vec<String>,
    #[serde(rename = "rt")]
    pub registered_time: i64,
    #[serde(rename = "et")]
    pub expiration_time: i64,
    #[serde(rename = "si")]
    pub supply_info: Option<SupplyInfo>,
    #[serde(rename = "dsi")]
    pub dynamic_start_info: Option<DynamicLocationInfo>,
    #[serde(rename = "dei")]
    pub dynamic_end_info: Option<DynamicLocationInfo>,
    #[serde(rename = "ddi")]
    pub dynamic_delivered_info: Option<DynamicLocationInfo>,
    #[serde(rename = "dmi")]
    pub dynamic_mission_info: Option<DynamicMissionInfo>,
    #[serde(rename = "b")]
    pub baggages: Option<Vec<Baggage>>,
}
