use super::{
    localized_text_resource::LocalizedTextResource, reference::Ref, resource::Resource, LoadContext,
};

#[derive(Debug, Clone)]
pub struct GameListItemBase {
    base: Resource,
    pub localized_name: Ref<LocalizedTextResource>,
    pub localized_description: Ref<LocalizedTextResource>,
    pub id: u32,
    pub name_code: u32,
}

impl super::Read for GameListItemBase {
    fn read(
        reader: &mut binary_reader::BinaryReader,
        context: &mut LoadContext,
    ) -> Result<Self, anyhow::Error> {
        let base = Resource::read(reader, context)?;
        let localized_name = Ref::<LocalizedTextResource>::read(reader, context)?;
        let localized_description = Ref::<LocalizedTextResource>::read(reader, context)?;
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
