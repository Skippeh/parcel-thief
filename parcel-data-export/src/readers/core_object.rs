use std::ops::{Deref, DerefMut};

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

impl Deref for CoreObject {
    type Target = RTTIRefObject;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for CoreObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
