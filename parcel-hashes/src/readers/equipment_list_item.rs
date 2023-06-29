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
        context: &mut super::LoadContext,
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

impl AsRef<GameListItemBaseWithIcon> for EquipmentListItem {
    fn as_ref(&self) -> &GameListItemBaseWithIcon {
        &self.base
    }
}

impl AsMut<GameListItemBaseWithIcon> for EquipmentListItem {
    fn as_mut(&mut self) -> &mut GameListItemBaseWithIcon {
        &mut self.base
    }
}
