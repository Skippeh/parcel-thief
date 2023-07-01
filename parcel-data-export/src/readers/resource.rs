use std::ops::{Deref, DerefMut};

use super::{core_object::CoreObject, LoadContext};

#[derive(Debug, Clone)]
pub struct Resource {
    base: CoreObject,
}

impl super::Read for Resource {
    fn read(
        reader: &mut binary_reader::BinaryReader,
        context: &mut LoadContext,
    ) -> Result<Self, anyhow::Error> {
        let base = CoreObject::read(reader, context)?;
        Ok(Self { base })
    }
}

impl Deref for Resource {
    type Target = CoreObject;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for Resource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
