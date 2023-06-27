use super::{game_list_item_base::GameListItemBase, string::DSString, ListItemColor};

#[derive(Debug)]
pub struct GameListItemBaseWithIcon {
    base: GameListItemBase,
    pub ui_texture_base_name: String,
    pub color: ListItemColor,
}

impl super::Read for GameListItemBaseWithIcon {
    fn read(reader: &mut binary_reader::BinaryReader) -> Result<Self, anyhow::Error> {
        let base = GameListItemBase::read(reader)?;
        let ui_texture_base_name = DSString::read(reader)?.into();
        let color = ListItemColor::read(reader)?;

        Ok(Self {
            base,
            ui_texture_base_name,
            color,
        })
    }
}

impl AsRef<GameListItemBase> for GameListItemBaseWithIcon {
    fn as_ref(&self) -> &GameListItemBase {
        &self.base
    }
}

impl AsMut<GameListItemBase> for GameListItemBaseWithIcon {
    fn as_mut(&mut self) -> &mut GameListItemBase {
        &mut self.base
    }
}
