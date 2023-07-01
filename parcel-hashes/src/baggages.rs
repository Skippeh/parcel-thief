use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    str::FromStr,
};

use serde::Serialize;

use crate::{
    readers::{
        baggage_list_item::{
            BaggageCaseType, BaggageListItem, ContentsDamageType, ContentsType, VolumeType,
        },
        game_list_item_base::GameListItemBase,
        localized_text_resource::Language,
        LoadContext, RTTITypeHash,
    },
    ObjectMetaData,
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Baggage {
    pub name_hash: u32,
    pub object_metadata: ObjectMetaData,
    pub baggage_metadata: BaggageMetaData,
    pub names: BTreeMap<Language, String>,
    pub descriptions: BTreeMap<Language, String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BaggageMetaData {
    pub type_case: BaggageCaseType,
    pub type_contents_damage: ContentsDamageType,
    pub type_contents: ContentsType,
    pub type_volume: VolumeType,
    pub amount: u32,
    pub sub_amount: u32,
    pub weight: f32,
    pub durability_contents: u32,
    pub durability_case: u32,
    pub initial_durability_contents: u32,
    pub initial_durability_case: u32,
    pub mission_id: u32,
}

impl From<&BaggageListItem> for BaggageMetaData {
    fn from(value: &BaggageListItem) -> Self {
        Self {
            type_case: value.type_case,
            type_contents_damage: value.type_contents_damage,
            type_contents: value.type_contents,
            type_volume: value.type_volume,
            amount: value.amount,
            sub_amount: value.sub_amount,
            weight: value.weight,
            durability_contents: value.durability_contents,
            durability_case: value.durability_case,
            initial_durability_contents: value.initial_durability_contents,
            initial_durability_case: value.initial_durability_case,
            mission_id: value.mission_id,
        }
    }
}

pub fn read_baggages(
    load_context: &mut LoadContext,
    out_baggages: &mut Vec<Baggage>,
) -> Result<(), anyhow::Error> {
    out_baggages.append(&mut read_baggages_from_file(
        &PathBuf::from_str("ds/catalogue/baggages/baggage_equipment.core")
            .expect("Path should always be valid"),
        load_context,
    )?);

    out_baggages.append(&mut read_baggages_from_file(
        &PathBuf::from_str("ds/catalogue/baggages/baggage_item.core")
            .expect("Path should always be valid"),
        load_context,
    )?);

    out_baggages.append(&mut read_baggages_from_file(
        &PathBuf::from_str("ds/catalogue/baggages/baggage_mission.core")
            .expect("Path should always be valid"),
        load_context,
    )?);

    out_baggages.append(&mut read_baggages_from_file(
        &PathBuf::from_str("ds/catalogue/baggages/baggage_rawmaterial.core")
            .expect("Path should always be valid"),
        load_context,
    )?);

    out_baggages.append(&mut read_baggages_from_file(
        &PathBuf::from_str("ds/catalogue/baggages/baggage_special.core")
            .expect("Path should always be valid"),
        load_context,
    )?);

    out_baggages.append(&mut read_baggages_from_file(
        &PathBuf::from_str("ds/catalogue/baggages/baggage_weapon.core")
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

    // Add all BaggageListItems
    for rtti_item in file.get_objects(&RTTITypeHash::BaggageListItem)? {
        let item = rtti_item
            .as_baggage_list_item()
            .expect("Entry should be a BaggageListItem");

        let (names, descriptions) = get_names_and_descriptions(item);
        let baggage_metadata = item.into();
        let object_metadata = ObjectMetaData {
            uuid: rtti_item.object_uuid().to_string(),
        };

        baggages.push(Baggage {
            name_hash: item.name_code,
            names,
            descriptions,
            baggage_metadata,
            object_metadata,
        });
    }

    Ok(baggages)
}

fn get_names_and_descriptions(
    item: &GameListItemBase,
) -> (BTreeMap<Language, String>, BTreeMap<Language, String>) {
    let mut names = BTreeMap::new();
    let mut descriptions = BTreeMap::new();

    // load localization from ref
    let name_res = &item.localized_name.value;
    let desc_res = &item.localized_description.value;

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
    (names, descriptions)
}
