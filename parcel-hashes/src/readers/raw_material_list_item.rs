use super::{game_list_item_base_with_icon::GameListItemBaseWithIcon, LoadContext};

#[derive(Debug, Clone)]
pub struct RawMaterialListItem {
    base: GameListItemBaseWithIcon,
    pub raw_material_type: RawMaterialType,
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

impl super::Read for RawMaterialType {
    fn read(
        reader: &mut binary_reader::BinaryReader,
        _: &mut LoadContext,
    ) -> Result<Self, anyhow::Error> {
        match reader.read_u16()? {
            0 => Ok(Self::Crystal),
            1 => Ok(Self::Resin),
            2 => Ok(Self::Metal),
            3 => Ok(Self::Ceramic),
            4 => Ok(Self::ChemicalSubstance),
            5 => Ok(Self::SpecialAlloy),
            other => anyhow::bail!("Unknown raw material variant: {other}"),
        }
    }
}

impl super::ReadRTTIType for RawMaterialListItem {
    fn rtti_type() -> super::RTTITypeHash {
        super::RTTITypeHash::RawMaterialListItem
    }
}

impl super::Read for RawMaterialListItem {
    fn read(
        reader: &mut binary_reader::BinaryReader,
        context: &mut LoadContext,
    ) -> Result<Self, anyhow::Error> {
        let base = GameListItemBaseWithIcon::read(reader, context)?;
        let raw_material_type = RawMaterialType::read(reader, context)?;

        Ok(Self {
            base,
            raw_material_type,
        })
    }
}

impl AsRef<GameListItemBaseWithIcon> for RawMaterialListItem {
    fn as_ref(&self) -> &GameListItemBaseWithIcon {
        &self.base
    }
}

impl AsMut<GameListItemBaseWithIcon> for RawMaterialListItem {
    fn as_mut(&mut self) -> &mut GameListItemBaseWithIcon {
        &mut self.base
    }
}
