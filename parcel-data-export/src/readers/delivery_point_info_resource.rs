use std::ops::{Deref, DerefMut};

use int_enum::IntEnum;
use serde::Serialize;

use super::{
    array::Array,
    localized_text_resource::LocalizedTextResource,
    mission_static_abstract_point_resource::MissionStaticAbstractPointResource,
    reference::{Ref, UnresolvedRef},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntEnum, Serialize)]
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

impl super::Read for Area {
    fn read(
        reader: &mut binary_reader::BinaryReader,
        _: &mut super::LoadContext,
    ) -> Result<Self, anyhow::Error> {
        let val = reader.read_u16()?;
        Ok(Area::from_int(val)?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntEnum, Serialize)]
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

impl super::Read for ConstructionPointType {
    fn read(
        reader: &mut binary_reader::BinaryReader,
        _: &mut super::LoadContext,
    ) -> Result<Self, anyhow::Error> {
        let val = reader.read_u8()?;
        Ok(ConstructionPointType::from_int(val)?)
    }
}

#[derive(Debug, Clone)]
pub struct WorldTransform {
    pub position: (f64, f64, f64),
    pub orientation: Vec<u8>, // RotMatrix (36 bytes)
}

impl super::Read for WorldTransform {
    fn read(
        reader: &mut binary_reader::BinaryReader,
        _: &mut super::LoadContext,
    ) -> Result<Self, anyhow::Error> {
        let position = (reader.read_f64()?, reader.read_f64()?, reader.read_f64()?);
        let orientation = reader.read_bytes(36)?.to_vec();

        Ok(WorldTransform {
            position,
            orientation,
        })
    }
}

#[derive(Debug, Clone)]
pub struct DeliveryPointInfoResource {
    base: MissionStaticAbstractPointResource,
    pub description_text: Ref<LocalizedTextResource>,
    pub parent_delivery_point: UnresolvedRef, // Ref<DeliveryPointInfoResource>
    pub house_hold: UnresolvedRef,            // Ref<DSHouseHoldInfoResource>
    pub terminal_operation_graph: UnresolvedRef, // Ref<DSTerminalGraphResource>
    pub private_room_operation_graph: UnresolvedRef, // Ref<DSPrivateRoomGraphResource>
    pub locator: UnresolvedRef,               // Ref<DSDeliveryPointLocator>
    pub inside_security_facts: Array<UnresolvedRef>, // Ref<BooleanFact>
    pub area: Area,
    pub delivery_point_type: ConstructionPointType,
    pub delivery_point_info_flag: u32,
    pub delivery_point_locator_id: i32,
    pub traffic: u32,
    pub order_in_list: u32,
    pub world_transform: WorldTransform,
    pub ui_resource: UnresolvedRef, // Ref<DSUIConstructionPointResource>
    pub extend_description_text: UnresolvedRef, // Ref<DSRewritableDeliveryPointInfoText>
    pub special_report_on_place: UnresolvedRef, // Ref<DSMissionSpecialReportResource>
}

impl super::Read for DeliveryPointInfoResource {
    fn read(
        reader: &mut binary_reader::BinaryReader,
        context: &mut super::LoadContext,
    ) -> Result<Self, anyhow::Error> {
        let base = MissionStaticAbstractPointResource::read(reader, context)?;
        let description_text = Ref::<LocalizedTextResource>::read(reader, context)?;
        let parent_delivery_point = UnresolvedRef::read(reader, context)?;
        let house_hold = UnresolvedRef::read(reader, context)?;
        let terminal_operation_graph = UnresolvedRef::read(reader, context)?;
        let private_room_operation_graph = UnresolvedRef::read(reader, context)?;
        let locator = UnresolvedRef::read(reader, context)?;
        let inside_security_facts = Array::read(reader, context)?;
        let area = Area::read(reader, context)?;
        let delivery_point_type = ConstructionPointType::read(reader, context)?;
        let delivery_point_info_flag = reader.read_u32()?;
        let delivery_point_locator_id = reader.read_i32()?;
        let traffic = reader.read_u32()?;
        let order_in_list = reader.read_u32()?;
        let world_transform = WorldTransform::read(reader, context)?;
        let ui_resource = UnresolvedRef::read(reader, context)?;
        let extend_description_text = UnresolvedRef::read(reader, context)?;
        let special_report_on_place = UnresolvedRef::read(reader, context)?;

        Ok(Self {
            base,
            description_text,
            parent_delivery_point,
            house_hold,
            terminal_operation_graph,
            private_room_operation_graph,
            locator,
            inside_security_facts,
            area,
            delivery_point_type,
            delivery_point_info_flag,
            delivery_point_locator_id,
            traffic,
            order_in_list,
            world_transform,
            ui_resource,
            extend_description_text,
            special_report_on_place,
        })
    }
}

impl Deref for DeliveryPointInfoResource {
    type Target = MissionStaticAbstractPointResource;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for DeliveryPointInfoResource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
