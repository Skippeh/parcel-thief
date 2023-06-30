use std::ops::{Deref, DerefMut};

use binary_reader::BinaryReader;

use super::{game_list_item_base::GameListItemBase, string::DSString, LoadContext};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ListItemColor {
    Red,
    Yellow,
    Blue,
    Gray,
    Orange,
    Purple,
}

impl super::Read for ListItemColor {
    fn read(reader: &mut BinaryReader, _: &mut LoadContext) -> Result<Self, anyhow::Error> {
        Ok(match reader.read_u8()? {
            0 => ListItemColor::Red,
            1 => ListItemColor::Yellow,
            2 => ListItemColor::Blue,
            3 => ListItemColor::Gray,
            4 => ListItemColor::Orange,
            5 => ListItemColor::Purple,
            other => anyhow::bail!("Unknown color variant: {other}"),
        })
    }
}

#[derive(Debug, Clone)]
pub struct GameListItemBaseWithIcon {
    base: GameListItemBase,
    pub ui_texture_base_name: String,
    pub color: ListItemColor,
}

impl super::Read for GameListItemBaseWithIcon {
    fn read(
        reader: &mut binary_reader::BinaryReader,
        context: &mut LoadContext,
    ) -> Result<Self, anyhow::Error> {
        let base = GameListItemBase::read(reader, context)?;
        let ui_texture_base_name = DSString::read(reader, context)?.into();
        let color = ListItemColor::read(reader, context)?;

        Ok(Self {
            base,
            ui_texture_base_name,
            color,
        })
    }
}

impl Deref for GameListItemBaseWithIcon {
    type Target = GameListItemBase;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for GameListItemBaseWithIcon {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
