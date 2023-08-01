mod baggages;
pub mod qpid_areas;
mod readers;

use std::{collections::BTreeMap, path::PathBuf};

use anyhow::Context;
use clap::Parser;
use parcel_game_data::{Baggage, QpidArea};
use readers::LoadContext;
use serde::{Deserialize, Serialize};

#[derive(Debug, clap::Parser)]
struct Options {
    #[clap(id = "EXTRACTED_DATA_DIR")]
    data_directory: PathBuf,
    output_path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GameDataExport {
    pub baggages: BTreeMap<u32, Baggage>,
    pub qpid_areas: BTreeMap<i32, QpidArea>,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Options::parse();
    let mut output = GameDataExport::default();
    let mut load_context = LoadContext::new(args.data_directory.clone());

    baggages::read_baggages(&mut load_context, &mut output.baggages)
        .context("Could not read baggages")?;
    qpid_areas::read_qpid_areas(&mut load_context, &mut output.qpid_areas)
        .context("Could not read qpid areas")?;

    let new_file = std::fs::File::create(args.output_path)?;
    serde_json::to_writer_pretty(new_file, &output)?;

    Ok(())
}
