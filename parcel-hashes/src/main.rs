pub mod readers;

use std::{
    collections::BTreeMap,
    fs::File,
    path::{Path, PathBuf},
};

use clap::Parser;
use readers::{CoreFile, localized_text_resource::Language};
use serde::Serialize;

#[derive(Debug, clap::Parser)]
struct Options {
    #[clap(id = "EXTRACTED_DATA_DIR")]
    data_directory: PathBuf,
    output_path: PathBuf,
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
