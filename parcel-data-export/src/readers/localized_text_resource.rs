use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use enum_iterator::all;
use parcel_game_data::Language;

use super::{resource::Resource, LoadContext};

#[derive(Debug, Clone)]
pub struct LocalizedTextResource {
    base: Resource,
    pub languages: HashMap<Language, DSLocalizedText>,
}

#[derive(Debug, Clone)]
pub struct DSLocalizedText {
    pub text: String,
    pub notes: String,
    pub flags: u8,
}

impl super::ReadRTTIType for LocalizedTextResource {
    fn rtti_type() -> super::RTTITypeHash {
        super::RTTITypeHash::LocalizedTextResource
    }
}

impl super::Read for LocalizedTextResource {
    fn read(
        reader: &mut binary_reader::BinaryReader,
        context: &mut LoadContext,
    ) -> Result<Self, anyhow::Error> {
        let base = Resource::read(reader, context)?;
        let mut languages = HashMap::new();

        for lang in all::<Language>() {
            if lang == Language::Unknown {
                continue;
            }

            let ds_text = DSLocalizedText::read(reader, context)?;
            languages.insert(lang, ds_text);
        }

        Ok(Self { base, languages })
    }
}

impl super::Read for DSLocalizedText {
    fn read(
        reader: &mut binary_reader::BinaryReader,
        _: &mut LoadContext,
    ) -> Result<Self, anyhow::Error> {
        let text_len = reader.read_u16()?;
        let text = String::from_utf8(reader.read_bytes(text_len as usize)?.to_vec())?;
        let notes_len = reader.read_u16()?;
        let notes = String::from_utf8(reader.read_bytes(notes_len as usize)?.to_vec())?;
        let flags = reader.read_u8()?;

        Ok(Self { text, notes, flags })
    }
}

impl Deref for LocalizedTextResource {
    type Target = Resource;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for LocalizedTextResource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
