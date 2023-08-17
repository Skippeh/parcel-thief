use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[cfg(feature = "ts")]
use typescript_type_def::TypeDef;

use crate::{Language, ObjectMetaData};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Baggage {
    pub name_hash: u32,
    pub object_metadata: ObjectMetaData,
    pub baggage_metadata: BaggageMetaData,
    pub names: BTreeMap<Language, String>,
    pub descriptions: BTreeMap<Language, String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaggageMetaData {
    pub type_case: BaggageCaseType,
    pub type_contents_damage: ContentsDamageType,
    pub type_contents: ContentsType,
    pub type_volume: VolumeType,
    pub amount: u32,
    pub sub_amount: u32,
    pub weight: f32,
    pub durability_contents: u32,
    pub durability_case: u32,
    pub initial_durability_contents: u32,
    pub initial_durability_case: u32,
    pub mission_id: u32,
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(TypeDef))]
#[serde(rename_all = "camelCase")]
#[repr(u8)]
pub enum ContentsType {
    Commodity,
    Weapon,
    Equipment,
    Special,
    RawMaterial,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[repr(u8)]
pub enum VolumeType {
    Small,
    Medium,
    Large,
    ExtraLarge,
    Human,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TypeOfConsume {
    None,
    Once,
    LikeBattery,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EffectivenessType {
    RecoveryOfStamina,
    RecoveryOfBlood,
    RecoveryOfCase,
    RecoveryOfBattery,
    Shoes,
    BtLight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum EquipmentType {
    Suits,
    Mask,
    Boots,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RawMaterialType {
    Crystal,
    Resin,
    Metal,
    Ceramic,
    ChemicalSubstance,
    SpecialAlloy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WeaponType {
    Main,
    Sub,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WeaponCategory {
    Gun,
    Throwing,
    Placement,
}
