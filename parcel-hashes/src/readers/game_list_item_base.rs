use super::{localized_text_resource::LocalizedTextResource, reference::Ref, resource::Resource};

#[derive(Debug)]
pub struct GameListItemBase {
    base: Resource,
    pub localized_name: Option<Ref<LocalizedTextResource>>,
    pub localized_description: Option<Ref<LocalizedTextResource>>,
    pub id: u32,
    pub name_code: u32,
}

impl super::Read for GameListItemBase {
    fn read(reader: &mut binary_reader::BinaryReader) -> Result<Self, anyhow::Error> {
        let base = Resource::read(reader)?;
        let localized_name = Option::<Ref<LocalizedTextResource>>::read(reader)?;
        let localized_description = Option::<Ref<LocalizedTextResource>>::read(reader)?;
        let id = reader.read_u32()?;
        let name_code = reader.read_u32()?;

        Ok(Self {
            base,
            localized_name,
            localized_description,
            id,
            name_code,
        })
    }
}

impl AsRef<Resource> for GameListItemBase {
    fn as_ref(&self) -> &Resource {
        &self.base
    }
}

impl AsMut<Resource> for GameListItemBase {
    fn as_mut(&mut self) -> &mut Resource {
        &mut self.base
    }
}
