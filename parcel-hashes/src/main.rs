mod baggages;
mod readers;

use std::{collections::BTreeMap, path::PathBuf};

use baggages::Baggage;
use clap::Parser;
use readers::{localized_text_resource::Language, LoadContext};
use serde::Serialize;

#[derive(Debug, clap::Parser)]
struct Options {
    #[clap(id = "EXTRACTED_DATA_DIR")]
    data_directory: PathBuf,
    output_path: PathBuf,
}

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
struct Output {
    baggages: Vec<Baggage>,
    qpid_areas: Vec<QpidArea>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectMetaData {
    pub uuid: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct QpidArea {
    pub qpid_id: u32,
    pub names: BTreeMap<Language, String>,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Options::parse();
    let mut output = Output::default();
    let mut load_context = LoadContext::new(args.data_directory.clone());

    baggages::read_baggages(&mut load_context, &mut output.baggages)?;

    let new_file = std::fs::File::create(args.output_path)?;
    serde_json::to_writer_pretty(new_file, &output)?;

    Ok(())
}
