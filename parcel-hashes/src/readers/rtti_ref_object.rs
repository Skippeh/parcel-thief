use uuid::Uuid;

use super::{rtti_object::RTTIObject, LoadContext};

#[derive(Debug, Clone)]
pub struct RTTIRefObject {
    base: RTTIObject,
    pub object_uuid: Uuid,
}

impl super::Read for RTTIRefObject {
    fn read(
        reader: &mut binary_reader::BinaryReader,
        context: &mut LoadContext,
    ) -> Result<Self, anyhow::Error> {
        let base = RTTIObject::read(reader, context)?;
        let uuid_bytes = reader.read_bytes(16)?;

        Ok(RTTIRefObject {
            base,
            object_uuid: Uuid::from_slice_le(uuid_bytes)?,
        })
    }
}

impl AsRef<RTTIObject> for RTTIRefObject {
    fn as_ref(&self) -> &RTTIObject {
        &self.base
    }
}

impl AsMut<RTTIObject> for RTTIRefObject {
    fn as_mut(&mut self) -> &mut RTTIObject {
        &mut self.base
    }
}
