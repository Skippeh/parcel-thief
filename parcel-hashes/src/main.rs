pub mod readers;

use std::{
    collections::BTreeMap,
    fs::File,
    path::{Path, PathBuf},
};

use clap::Parser;
use readers::CoreFile;
use serde::Serialize;

#[derive(Debug, clap::Parser)]
struct Options {
    #[clap(id = "EXTRACTED_DATA_DIR")]
    data_directory: PathBuf,
    output_path: PathBuf,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, enum_iterator::Sequence)]
#[repr(i32)]
enum Language {
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

#[derive(Debug, Serialize, Default)]
struct Output {
    baggages: Vec<Baggage>,
    qpid_areas: Vec<QpidArea>,
}

#[derive(Debug, Serialize)]
struct Baggage {
    pub name_hash: i32,
    pub names: BTreeMap<Language, String>,
}

#[derive(Debug, Serialize)]
struct QpidArea {
    pub qpid_id: i32,
    pub names: BTreeMap<Language, String>,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Options::parse();
    let mut output = Output::default();

    read_baggages(&args.data_directory, &mut output.baggages)?;

    let new_file = std::fs::File::create(args.output_path)?;
    serde_json::to_writer_pretty(new_file, &output)?;

    Ok(())
}

fn read_baggages(data_directory: &Path, baggages: &mut Vec<Baggage>) -> Result<(), anyhow::Error> {
    let raw_materials_file = data_directory.join("ds/catalogue/things/rawmaterial.core");
    let mut file = File::open(raw_materials_file)?;
    let file = CoreFile::from_file(&mut file)?;

    dbg!(file);

    Ok(())
}
