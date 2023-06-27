use super::resource::Resource;

#[derive(Debug)]
pub struct LocalizedTextResource {
    base: Resource,
    pub text: String,
    pub notes: String,
    pub flags: u8,
}

impl super::ReadRTTIType for LocalizedTextResource {
    fn rtti_type(&self) -> super::RTTITypeHash {
        super::RTTITypeHash::LocalizedTextResource
    }
}

impl super::Read for LocalizedTextResource {
    fn read(reader: &mut binary_reader::BinaryReader) -> Result<Self, anyhow::Error> {
        let base = Resource::read(reader)?;
        let text_len = reader.read_u16()?;
        let text = String::from_utf8(reader.read_bytes(text_len as usize)?.to_vec())?;
        let notes_len = reader.read_u16()?;
        let notes = String::from_utf8(reader.read_bytes(notes_len as usize)?.to_vec())?;
        let flags = reader.read_u8()?;

        Ok(Self {
            base,
            text,
            notes,
            flags,
        })
    }
}

impl AsRef<Resource> for LocalizedTextResource {
    fn as_ref(&self) -> &Resource {
        &self.base
    }
}

impl AsMut<Resource> for LocalizedTextResource {
    fn as_mut(&mut self) -> &mut Resource {
        &mut self.base
    }
}
