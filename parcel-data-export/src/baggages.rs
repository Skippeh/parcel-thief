use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    str::FromStr,
};

use parcel_game_data::{Baggage, BaggageMetaData, Language, ObjectMetaData};

use crate::readers::{
    baggage_list_item::BaggageListItem, game_list_item_base::GameListItemBase, LoadContext,
    RTTITypeHash,
};

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
    out_baggages: &mut BTreeMap<u32, Baggage>,
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
) -> Result<BTreeMap<u32, Baggage>, anyhow::Error> {
    let mut baggages = BTreeMap::new();
    let file = load_context.load_file(path)?.clone(); // cloning is necessary to avoid borrowing issues. there is probably a better way but i don't know it

    // Add all BaggageListItems
    for rtti_item in file.get_objects(&RTTITypeHash::BaggageListItem)? {
        let item = rtti_item
            .as_baggage_list_item()
            .expect("Entry should be a BaggageListItem");

        let (names, descriptions) = get_names_and_descriptions(item, load_context)?;
        let baggage_metadata = item.into();
        let object_metadata = ObjectMetaData {
            uuid: rtti_item.object_uuid().to_string(),
        };

        baggages.insert(
            item.name_code,
            Baggage {
                name_hash: item.name_code,
                names,
                descriptions,
                baggage_metadata,
                object_metadata,
            },
        );
    }

    Ok(baggages)
}

fn get_names_and_descriptions(
    item: &GameListItemBase,
    load_context: &mut LoadContext,
) -> Result<(BTreeMap<Language, String>, BTreeMap<Language, String>), anyhow::Error> {
    let mut names = BTreeMap::new();
    let mut descriptions = BTreeMap::new();

    // load localization from ref
    item.localized_name.load_resolve(load_context)?;
    item.localized_description.load_resolve(load_context)?;
    let name_res = &item.localized_name.resolve(load_context)?;
    let desc_res = &item.localized_description.resolve(load_context)?;

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

    Ok((names, descriptions))
}
