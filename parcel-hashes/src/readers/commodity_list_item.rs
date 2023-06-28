use super::{game_list_item_base_with_icon::GameListItemBaseWithIcon, LoadContext};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TypeOfConsume {
    None,
    Once,
    LikeBattery,
}

impl super::Read for TypeOfConsume {
    fn read(
        reader: &mut binary_reader::BinaryReader,
        _: &mut LoadContext,
    ) -> Result<Self, anyhow::Error> {
        Ok(match reader.read_u16()? {
            0 => Self::None,
            1 => Self::Once,
            2 => Self::LikeBattery,
            other => anyhow::bail!("Unknown type of TypeOfConsume variant: {other}"),
        })
    }
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

impl super::Read for EffectivenessType {
    fn read(
        reader: &mut binary_reader::BinaryReader,
        _: &mut LoadContext,
    ) -> Result<Self, anyhow::Error> {
        Ok(match reader.read_u16()? {
            0 => Self::RecoveryOfStamina,
            1 => Self::RecoveryOfBlood,
            2 => Self::RecoveryOfCase,
            3 => Self::RecoveryOfBattery,
            4 => Self::Shoes,
            5 => Self::BtLight,
            other => anyhow::bail!("Unknown type of EffectivenessType variant: {other}"),
        })
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CommodityListItem {
    base: GameListItemBaseWithIcon,
    type_of_consume: TypeOfConsume,
    max_amount_in_stock: u32,
    max_amount_for_player: u32,
    type_of_effectiveness: EffectivenessType,
    effective_time: u32,
    effective_point: u32,
}

impl super::ReadRTTIType for CommodityListItem {
    fn rtti_type() -> super::RTTITypeHash {
        super::RTTITypeHash::CommodityListItem
    }
}

impl super::Read for CommodityListItem {
    fn read(
        reader: &mut binary_reader::BinaryReader,
        context: &mut LoadContext,
    ) -> Result<Self, anyhow::Error> {
        let base = GameListItemBaseWithIcon::read(reader, context)?;
        let type_of_consume = TypeOfConsume::read(reader, context)?;
        let max_amount_in_stock = reader.read_u32()?;
        let max_amount_for_player = reader.read_u32()?;
        let type_of_effectiveness = EffectivenessType::read(reader, context)?;
        let effective_time = reader.read_u32()?;
        let effective_point = reader.read_u32()?;
        Ok(Self {
            base,
            type_of_consume,
            max_amount_in_stock,
            max_amount_for_player,
            type_of_effectiveness,
            effective_time,
            effective_point,
        })
    }
}

impl AsRef<GameListItemBaseWithIcon> for CommodityListItem {
    fn as_ref(&self) -> &GameListItemBaseWithIcon {
        &self.base
    }
}

impl AsMut<GameListItemBaseWithIcon> for CommodityListItem {
    fn as_mut(&mut self) -> &mut GameListItemBaseWithIcon {
        &mut self.base
    }
}
