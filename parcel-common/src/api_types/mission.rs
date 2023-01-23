use serde::{Deserialize, Serialize};

use super::area::AreaHash;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
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

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum MissionType {
    #[serde(rename = "Delivery")]
    Delivery = 0,
    #[serde(rename = "Collect")]
    Collect = 1,
    #[serde(rename = "Lost_object")]
    LostObject = 2,
    #[serde(rename = "Supply")]
    Supply = 3,
    #[serde(rename = "Special")]
    Special = 4,
    #[serde(rename = "Free")]
    Free = 5,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SupplyInfo {
    #[serde(rename = "h")]
    pub item_hash: i64,
    #[serde(rename = "n")]
    pub amount: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DynamicMissionInfo {
    #[serde(rename = "ch")]
    pub client_name_hash: i32,
    #[serde(rename = "rh")]
    pub reward_name_hash: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
pub struct AmmoInfo {
    #[serde(rename = "aid")]
    pub ammo_id: String,
    #[serde(rename = "cc")]
    pub clip_count: i16,
    #[serde(rename = "c")]
    pub count: i16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    #[serde(rename = "ret")]
    pub is_returned: bool,
    #[serde(rename = "am")]
    pub ammo_info: AmmoInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
