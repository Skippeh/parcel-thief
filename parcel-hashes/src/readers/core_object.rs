use super::{rtti_ref_object::RTTIRefObject, LoadContext};

#[derive(Debug, Clone)]
pub struct CoreObject {
    base: RTTIRefObject,
}

impl super::Read for CoreObject {
    fn read(
        reader: &mut binary_reader::BinaryReader,
        context: &mut LoadContext,
    ) -> Result<Self, anyhow::Error> {
        let base = RTTIRefObject::read(reader, context)?;
        Ok(CoreObject { base })
    }
}

impl AsRef<RTTIRefObject> for CoreObject {
    fn as_ref(&self) -> &RTTIRefObject {
        &self.base
    }
}

impl AsMut<RTTIRefObject> for CoreObject {
    fn as_mut(&mut self) -> &mut RTTIRefObject {
        &mut self.base
    }
}
