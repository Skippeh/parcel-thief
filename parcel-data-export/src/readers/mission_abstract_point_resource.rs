use std::ops::{Deref, DerefMut};

use super::{resource::Resource, LoadContext};

#[derive(Debug, Clone)]
pub struct MissionAbstractPointResource {
    base: Resource,
}

impl super::Read for MissionAbstractPointResource {
    fn read(
        reader: &mut binary_reader::BinaryReader,
        context: &mut LoadContext,
    ) -> Result<Self, anyhow::Error> {
        let base = Resource::read(reader, context)?;
        Ok(Self { base })
    }
}

impl Deref for MissionAbstractPointResource {
    type Target = Resource;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for MissionAbstractPointResource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
