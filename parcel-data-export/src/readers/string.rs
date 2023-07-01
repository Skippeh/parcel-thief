use std::ops::{Deref, DerefMut};

use super::LoadContext;

#[derive(Debug, Clone)]
pub struct DSString(String);

impl super::Read for DSString {
    fn read(
        reader: &mut binary_reader::BinaryReader,
        _: &mut LoadContext,
    ) -> Result<Self, anyhow::Error> {
        let len = reader.read_u32()?;

        if len > 0 {
            let _crc = reader.read_u32();
            let utf8_bytes = reader.read_bytes(len as usize)?.to_vec();
            let str = String::from_utf8(utf8_bytes)?;

            // todo: validate crc32c (uses a custom implementation)

            Ok(Self(str))
        } else {
            Ok(Self("".to_string()))
        }
    }
}

impl Deref for DSString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DSString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<DSString> for String {
    fn from(val: DSString) -> Self {
        val.0
    }
}
