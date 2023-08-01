use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Serialize,
    Deserialize,
    enum_iterator::Sequence,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
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
    #[serde(rename = "es-419")]
    Latamsp = 17,
    #[serde(rename = "latampor")] // latin america portuguese (not sure what the language code is)
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
