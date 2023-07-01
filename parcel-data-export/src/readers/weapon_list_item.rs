use std::ops::{Deref, DerefMut};

use super::game_list_item_base_with_icon::GameListItemBaseWithIcon;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WeaponType {
    Main,
    Sub,
}

impl super::Read for WeaponType {
    fn read(
        reader: &mut binary_reader::BinaryReader,
        _: &mut super::LoadContext,
    ) -> Result<Self, anyhow::Error> {
        Ok(match reader.read_u16()? {
            0 => Self::Main,
            1 => Self::Sub,
            other => anyhow::bail!("Unknown weapon variant: {other}"),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WeaponCategory {
    Gun,
    Throwing,
    Placement,
}

impl super::Read for WeaponCategory {
    fn read(
        reader: &mut binary_reader::BinaryReader,
        _: &mut super::LoadContext,
    ) -> Result<Self, anyhow::Error> {
        Ok(match reader.read_u16()? {
            0 => Self::Gun,
            1 => Self::Throwing,
            2 => Self::Placement,
            other => anyhow::bail!("Unknown weapon category variant: {other}"),
        })
    }
}

#[derive(Debug, Clone)]
pub struct WeaponListItem {
    pub base: GameListItemBaseWithIcon,
    pub weapon_type: WeaponType,
    pub weapon_category: WeaponCategory,
    pub param_0: u16,
    pub param_1: u16,
    pub param_2: u16,
}

impl super::ReadRTTIType for WeaponListItem {
    fn rtti_type() -> super::RTTITypeHash {
        super::RTTITypeHash::WeaponListItem
    }
}

impl super::Read for WeaponListItem {
    fn read(
        reader: &mut binary_reader::BinaryReader,
        context: &mut super::LoadContext,
    ) -> Result<Self, anyhow::Error> {
        let base = GameListItemBaseWithIcon::read(reader, context)?;
        let weapon_type = WeaponType::read(reader, context)?;
        let weapon_category = WeaponCategory::read(reader, context)?;
        let param_0 = reader.read_u16()?;
        let param_1 = reader.read_u16()?;
        let param_2 = reader.read_u16()?;
        Ok(Self {
            base,
            weapon_type,
            weapon_category,
            param_0,
            param_1,
            param_2,
        })
    }
}

impl Deref for WeaponListItem {
    type Target = GameListItemBaseWithIcon;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for WeaponListItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
