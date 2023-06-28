pub mod readers;

use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    str::FromStr,
};

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
struct Output {
    baggages: Vec<Baggage>,
    qpid_areas: Vec<QpidArea>,
}

#[derive(Debug, Serialize)]
struct Baggage {
    pub name_hash: u32,
    pub names: BTreeMap<Language, String>,
    pub descriptions: BTreeMap<Language, String>,
}

#[derive(Debug, Serialize)]
struct QpidArea {
    pub qpid_id: u32,
    pub names: BTreeMap<Language, String>,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Options::parse();
    let mut output = Output::default();
    let mut load_context = LoadContext::new(args.data_directory.clone());

    read_baggages(&mut load_context, &mut output.baggages)?;

    let new_file = std::fs::File::create(args.output_path)?;
    serde_json::to_writer_pretty(new_file, &output)?;

    Ok(())
}

fn read_baggages(
    load_context: &mut LoadContext,
    out_baggages: &mut Vec<Baggage>,
) -> Result<(), anyhow::Error> {
    out_baggages.append(&mut read_baggages_from_file(
        &PathBuf::from_str("ds/catalogue/things/rawmaterial.core")
            .expect("Path should always be valid"),
        load_context,
    )?);

    Ok(())
}

fn read_baggages_from_file(
    path: &Path,
    load_context: &mut LoadContext,
) -> Result<Vec<Baggage>, anyhow::Error> {
    let mut baggages = Vec::new();
    let file = load_context.load_file(path)?;

    // Add all RawMaterialListItems
    for item in file.get_objects(&readers::RTTITypeHash::RawMaterialListItem)? {
        let item = item
            .as_raw_material_list_item()
            .expect("Entry should be a RawMaterialListItem");

        let mut names = BTreeMap::new();
        let mut descriptions = BTreeMap::new();

        // load localization from ref
        let name_res = &item.as_ref().as_ref().localized_name.value;
        let desc_res = &item.as_ref().as_ref().localized_description.value;

        if let Some(name_res) = name_res {
            for (lang, name) in &name_res
                .as_localized_text_resource()
                .expect("Name should always be LocalizedTextResource")
                .languages
            {
                names.insert(*lang, name.text.clone());
            }
        }

        if let Some(desc_res) = desc_res {
            for (lang, desc) in &desc_res
                .as_localized_text_resource()
                .expect("Description should always be LocalizedTextResource")
                .languages
            {
                descriptions.insert(*lang, desc.text.clone());
            }
        }

        baggages.push(Baggage {
            name_hash: item.as_ref().as_ref().name_code,
            names,
            descriptions,
        })
    }

    Ok(baggages)
}
