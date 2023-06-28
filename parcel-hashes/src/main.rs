pub mod readers;

use std::{
    collections::BTreeMap,
    fs::File,
    path::{Path, PathBuf},
};

use clap::Parser;
use readers::{CoreFile, localized_text_resource::Language, reference::ResolveRef};
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

    read_baggages(&args.data_directory, &mut output.baggages)?;

    let new_file = std::fs::File::create(args.output_path)?;
    serde_json::to_writer_pretty(new_file, &output)?;

    Ok(())
}

fn read_baggages(data_directory: &Path, baggages: &mut Vec<Baggage>) -> Result<(), anyhow::Error> {
    let raw_materials_file = data_directory.join("ds/catalogue/things/rawmaterial.core");
    read_baggages_from_file(&raw_materials_file)?;

    Ok(())
}

fn read_baggages_from_file(path: &Path) -> Result<Vec<Baggage>, anyhow::Error> {
    let mut baggages = Vec::new();
    let mut file = File::open(path)?;
    let file = CoreFile::from_file(&mut file)?;

    // Add all RawMaterialListItems
    for item in file.get_objects(&readers::RTTITypeHash::RawMaterialListItem)? {
        let item = item.as_raw_material_list_item().expect("Entry is a RawMaterialListItem");
        let mut names = BTreeMap::new();
        let mut descriptions = BTreeMap::new();

        // load localization from ref
        let name_res = item.as_ref().as_ref().localized_name.as_ref().map(|rf| rf.resolve_ref()).transpose()?;
        let desc_res = item.as_ref().as_ref().localized_description.as_ref().map(|rf| rf.resolve_ref()).transpose()?;

        if let Some(name_res) = name_res {
            for (lang, name) in &name_res.languages {
                names.insert(*lang, name.text.clone());
            }
        }

        if let Some(desc_res) = desc_res {
            for (lang, desc) in &desc_res.languages {
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