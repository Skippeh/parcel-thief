use std::ops::{Deref, DerefMut};

use super::game_list_item_base_with_icon::GameListItemBaseWithIcon;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum EquipmentType {
    Suits,
    Mask,
    Boots,
}

impl super::Read for EquipmentType {
    fn read(
        reader: &mut binary_reader::BinaryReader,
        _: &mut super::LoadContext,
    ) -> Result<Self, anyhow::Error> {
        Ok(match reader.read_u16()? {
            0 => Self::Suits,
            1 => Self::Mask,
            2 => Self::Boots,
            other => anyhow::bail!("Unknown EquipmentType variant: {other}"),
        })
    }
}

#[derive(Debug, Clone)]
pub struct EquipmentListItem {
    base: GameListItemBaseWithIcon,
    pub equipment_type: EquipmentType,
    pub param: u32,
    pub max_volume: u32,
    pub max_durability: u32,
}

impl super::Read for EquipmentListItem {
    fn read(
        reader: &mut binary_reader::BinaryReader,
        context: &mut super::LoadContext,
    ) -> Result<Self, anyhow::Error> {
        let base = GameListItemBaseWithIcon::read(reader, context)?;
        let equipment_type = EquipmentType::read(reader, context)?;
        let param = reader.read_u32()?;
        let max_volume = reader.read_u32()?;
        let max_durability = reader.read_u32()?;
        Ok(Self {
            base,
            equipment_type,
            param,
            max_volume,
            max_durability,
        })
    }
}

impl Deref for EquipmentListItem {
    type Target = GameListItemBaseWithIcon;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for EquipmentListItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
