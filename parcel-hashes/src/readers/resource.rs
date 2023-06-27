use super::core_object::CoreObject;

#[derive(Debug)]
pub struct Resource {
    base: CoreObject,
}

impl super::Read for Resource {
    fn read(reader: &mut binary_reader::BinaryReader) -> Result<Self, anyhow::Error> {
        let base = CoreObject::read(reader)?;
        Ok(Self { base })
    }
}

impl AsRef<CoreObject> for Resource {
    fn as_ref(&self) -> &CoreObject {
        &self.base
    }
}

impl AsMut<CoreObject> for Resource {
    fn as_mut(&mut self) -> &mut CoreObject {
        &mut self.base
    }
}
