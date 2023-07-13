use std::collections::BTreeMap;

use int_enum::IntEnum;
use serde::{Deserialize, Serialize};

use crate::Language;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QpidArea {
    pub qpid_id: i32,
    pub names: BTreeMap<Language, String>,
    pub metadata: QpidAreaMetaData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QpidAreaMetaData {
    pub order_in_list: u32,
    pub construction_type: ConstructionPointType,
    pub area: Area,
    pub location: (f64, f64, f64),
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntEnum, Serialize, Deserialize,
)]
#[serde(rename_all = "camelCase")]
#[repr(u16)]
pub enum Area {
    Area00 = 0,
    Area01 = 100,
    Area02 = 200,
    Area03 = 300,
    Area04 = 400,
    Warrior01 = 500,
    Warrior02 = 510,
    Warrior03 = 520,
    Beach01 = 600,
    Empty = 65535,
    Frange01 = 700,
    Nm01 = 1100,
    Nm02 = 1200,
    Nm04 = 1400,
    _Reserved0 = 10000,
    _Reserved1 = 10001,
    _Reserved2 = 10002,
    _Reserved3 = 10003,
    _Reserved4 = 10100,
    _Reserved5 = 10101,
    _Reserved6 = 10103,
    _Reserved7 = 10104,
    _Reserved8 = 10200,
    _Reserved9 = 15001,
    A = 15002,
    B = 20000,
    C = 30000,
    D = 10004,
    E = 10105,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntEnum, Serialize, Deserialize,
)]
#[serde(rename_all = "camelCase")]
#[repr(u8)]
pub enum ConstructionPointType {
    DeliveryBase = 0,
    PreppersShelter = 1,
    StageSafetyHouse = 2,
    PlayerSafetyHouse = 3,
    NetSafetyHouse = 4,
    StagePost = 5,
    PlayerPost = 6,
    NetPost = 7,
    StageWatchTower = 8,
    PlayerWatchTower = 9,
    NetWatchTower = 10,
    _Reserved0 = 11,
    _Reserved1 = 12,
    _Reserved2 = 13,
    StageCharger = 14,
    PlayerCharger = 15,
    NetCharger = 16,
    StageRainShelter = 17,
    PlayerRainShelter = 18,
    NetRainShelter = 19,
    MulePost = 20,
    StageZipline = 21,
    PlayerZipline = 22,
    NetZipline = 23,
    StageLadder = 24,
    PlayerLadder = 25,
    NetLadder = 26,
    StageFieldRope = 27,
    PlayerFieldRope = 28,
    NetFieldRope = 29,
    StageBridge30m = 30,
    PlayerBridge30m = 31,
    NetBridge30m = 32,
    StageBridge45m = 33,
    PlayerBridge45m = 34,
    NetBridge45m = 35,
    RoadRebuilder = 36,
    _Reserved3 = 37,
    _Reserved4 = 38,
    _Reserved5 = 39,
    _Reserved6 = 40,
    _Reserved7 = 41,
    _Reserved8 = 42,
    _Reserved9 = 43,
    _Reserved10 = 44,
    _Reserved11 = 45,
}
