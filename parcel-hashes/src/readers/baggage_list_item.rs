use serde::Serialize;

use super::{
    game_list_item_base::GameListItemBase, game_list_item_base_with_icon::GameListItemBaseWithIcon,
    reference::UnresolvedRef,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum BaggageAttribute {
    Locked,
    Personal,
    Dummy,
    Discarded,
    DummyBaggage,
    NonBaggage,
}

impl super::Read for BaggageAttribute {
    fn read(
        reader: &mut binary_reader::BinaryReader,
        _: &mut super::LoadContext,
    ) -> Result<Self, anyhow::Error> {
        Ok(match reader.read_u8()? {
            0 => Self::Locked,
            1 => Self::Personal,
            2 => Self::Dummy,
            3 => Self::Discarded,
            4 => Self::DummyBaggage,
            5 => Self::NonBaggage,
            other => anyhow::bail!("Unknown BaggageAttribute variant: {other}"),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(rename_all = "camelCase")]
#[repr(u8)]
pub enum BaggageCaseType {
    Normal,
    LiquidOnly,
    Weapon,
    Item,
    Equipment,
    BBPod,
    BodyBag,
    Dummy,
    Handcuffs,
    Material,
    Cart,
    ConstructionMachine,
    Ladder,
    Delicate,
    Rope,
    Vehicle,
    LivingThing,
    SmallDelicate,
    ToxicGas,
}

impl super::Read for BaggageCaseType {
    fn read(
        reader: &mut binary_reader::BinaryReader,
        _: &mut super::LoadContext,
    ) -> Result<Self, anyhow::Error> {
        Ok(match reader.read_u8()? {
            0 => Self::Normal,
            1 => Self::LiquidOnly,
            2 => Self::Weapon,
            3 => Self::Item,
            4 => Self::Equipment,
            5 => Self::BBPod,
            6 => Self::BodyBag,
            7 => Self::Dummy,
            8 => Self::Handcuffs,
            9 => Self::Material,
            10 => Self::Cart,
            11 => Self::ConstructionMachine,
            12 => Self::Ladder,
            13 => Self::Delicate,
            14 => Self::Rope,
            15 => Self::Vehicle,
            16 => Self::LivingThing,
            17 => Self::SmallDelicate,
            18 => Self::ToxicGas,
            other => anyhow::bail!("Unknown BaggageCaseType variant: {other}"),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(rename_all = "camelCase")]
#[repr(u8)]
pub enum ContentsDamageType {
    Normal,
    Fragile,
    Delicate,
    Danger,
    SensitiveToTimefall,
    Equipment,
    LivingThing,
    MustKeepHorizontally,
    Cool,
}

impl super::Read for ContentsDamageType {
    fn read(
        reader: &mut binary_reader::BinaryReader,
        _: &mut super::LoadContext,
    ) -> Result<Self, anyhow::Error> {
        Ok(match reader.read_u8()? {
            0 => Self::Normal,
            1 => Self::Fragile,
            2 => Self::Delicate,
            3 => Self::Danger,
            4 => Self::SensitiveToTimefall,
            5 => Self::Equipment,
            6 => Self::LivingThing,
            7 => Self::MustKeepHorizontally,
            8 => Self::Cool,
            other => anyhow::bail!("Unknown ContentsDamageType variant: {other}"),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(rename_all = "camelCase")]
#[repr(u8)]
pub enum ContentsType {
    Commodity,
    Weapon,
    Equipment,
    Special,
    RawMaterial,
}

impl super::Read for ContentsType {
    fn read(
        reader: &mut binary_reader::BinaryReader,
        _: &mut super::LoadContext,
    ) -> Result<Self, anyhow::Error> {
        Ok(match reader.read_u8()? {
            0 => Self::Commodity,
            1 => Self::Weapon,
            2 => Self::Equipment,
            3 => Self::Special,
            4 => Self::RawMaterial,
            other => anyhow::bail!("Unknown ContentsType variant: {other}"),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(rename_all = "camelCase")]
#[repr(u8)]
pub enum VolumeType {
    Small,
    Medium,
    Large,
    ExtraLarge,
    Human,
}

impl super::Read for VolumeType {
    fn read(
        reader: &mut binary_reader::BinaryReader,
        _: &mut super::LoadContext,
    ) -> Result<Self, anyhow::Error> {
        Ok(match reader.read_u8()? {
            0 => Self::Small,
            1 => Self::Medium,
            2 => Self::Large,
            3 => Self::ExtraLarge,
            4 => Self::Human,
            other => anyhow::bail!("Unknown VolumeType variant: {other}"),
        })
    }
}

#[derive(Debug, Clone)]
pub struct BaggageListItem {
    base: GameListItemBase,
    pub attribute_of_baggage: BaggageAttribute,
    pub type_case: BaggageCaseType,
    pub type_contents_damage: ContentsDamageType,
    pub type_contents: ContentsType,
    pub type_volume: VolumeType,
    pub contents: UnresolvedRef<GameListItemBaseWithIcon>,
    pub amount: u32,
    pub sub_amount: u32,
    pub weight: f32,
    pub durability_contents: u32,
    pub durability_case: u32,
    pub initial_durability_contents: u32,
    pub initial_durability_case: u32,
    pub mission_id: u32,
    pub rarity: u8,
}

impl super::ReadRTTIType for BaggageListItem {
    fn rtti_type() -> super::RTTITypeHash {
        super::RTTITypeHash::BaggageListItem
    }
}

impl super::Read for BaggageListItem {
    fn read(
        reader: &mut binary_reader::BinaryReader,
        context: &mut super::LoadContext,
    ) -> Result<Self, anyhow::Error> {
        let base = GameListItemBase::read(reader, context)?;
        let attribute_of_baggage = BaggageAttribute::read(reader, context)?;
        let case_type = BaggageCaseType::read(reader, context)?;
        let contents_damage_type = ContentsDamageType::read(reader, context)?;
        let contents_type = ContentsType::read(reader, context)?;
        let volume_type = VolumeType::read(reader, context)?;
        let contents = UnresolvedRef::read(reader, context)?;
        let amount = reader.read_u32()?;
        let sub_amount = reader.read_u32()?;
        let weight = reader.read_f32()?;
        let contents_durability = reader.read_u32()?;
        let case_durability = reader.read_u32()?;
        let initial_contents_durability = reader.read_u32()?;
        let initial_case_durability = reader.read_u32()?;
        let mission_id = reader.read_u32()?;
        let rarity = reader.read_u8()?;
        Ok(Self {
            base,
            attribute_of_baggage,
            type_case: case_type,
            type_contents_damage: contents_damage_type,
            type_contents: contents_type,
            type_volume: volume_type,
            contents,
            amount,
            sub_amount,
            weight,
            durability_contents: contents_durability,
            durability_case: case_durability,
            initial_durability_contents: initial_contents_durability,
            initial_durability_case: initial_case_durability,
            mission_id,
            rarity,
        })
    }
}

impl AsRef<GameListItemBase> for BaggageListItem {
    fn as_ref(&self) -> &GameListItemBase {
        &self.base
    }
}

impl AsMut<GameListItemBase> for BaggageListItem {
    fn as_mut(&mut self) -> &mut GameListItemBase {
        &mut self.base
    }
}
