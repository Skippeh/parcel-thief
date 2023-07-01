use std::ops::{Deref, DerefMut};

use super::{mission_abstract_point_resource::MissionAbstractPointResource, LoadContext};

#[derive(Debug, Clone)]
pub struct MissionStaticAbstractPointResource {
    base: MissionAbstractPointResource,
}

impl super::Read for MissionStaticAbstractPointResource {
    fn read(
        reader: &mut binary_reader::BinaryReader,
        context: &mut LoadContext,
    ) -> Result<Self, anyhow::Error> {
        let base = MissionAbstractPointResource::read(reader, context)?;
        Ok(Self { base })
    }
}

impl Deref for MissionStaticAbstractPointResource {
    type Target = MissionAbstractPointResource;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for MissionStaticAbstractPointResource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
