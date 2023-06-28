use std::collections::HashMap;

use enum_iterator::all;
use serde::Serialize;

use super::{resource::Resource, LoadContext};

#[derive(
    Debug, Serialize, enum_iterator::Sequence, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[repr(i32)]
pub enum Language {
    #[serde(rename = "unknown")]
    Unknown = 0,
    #[serde(rename = "en-us")]
    English = 1,
    #[serde(rename = "fr")]
    French = 2,
    #[serde(rename = "es")]
    Spanish = 3,
    #[serde(rename = "de")]
    German = 4,
    #[serde(rename = "it")]
    Italian = 5,
    #[serde(rename = "nl")]
    Dutch = 6,
    #[serde(rename = "pt")]
    Portuguese = 7,
    #[serde(rename = "zh-CHT")]
    ChineseTraditional = 8,
    #[serde(rename = "ko")]
    Korean = 9,
    #[serde(rename = "ru")]
    Russian = 10,
    #[serde(rename = "pl")]
    Polish = 11,
    #[serde(rename = "da")]
    Danish = 12,
    #[serde(rename = "fi")]
    Finnish = 13,
    #[serde(rename = "no")]
    Norwegian = 14,
    #[serde(rename = "sv")]
    Swedish = 15,
    #[serde(rename = "ja")]
    Japanese = 16,
    #[serde(rename = "latamsp")]
    Latamsp = 17,
    #[serde(rename = "latampor")]
    Latampor = 18,
    #[serde(rename = "tr")]
    Turkish = 19,
    #[serde(rename = "ar")]
    Arabic = 20,
    #[serde(rename = "zh-CN")]
    ChineseSimplified = 21,
    #[serde(rename = "en-uk")]
    EnglishUk = 22,
    #[serde(rename = "el")]
    Greek = 23,
    #[serde(rename = "cs")]
    Czech = 24,
    #[serde(rename = "hu")]
    Hungarian = 25,
}

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
