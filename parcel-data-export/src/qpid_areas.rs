use std::{collections::BTreeMap, ffi::OsStr, path::Path};

use parcel_game_data::{ConstructionPointType, Language, QpidArea, QpidAreaMetaData};

use crate::readers::{
    delivery_point_info_resource::DeliveryPointInfoResource, LoadContext, RTTITypeHash,
};

pub fn read_qpid_areas(
    load_context: &mut LoadContext,
    qpid_areas: &mut BTreeMap<i32, QpidArea>,
) -> Result<(), anyhow::Error> {
    let location_dir = load_context.get_absolute_path(&Path::new("ds").join("location"));

    // Iterate each folder in location dir, which consists of a folder for each area/level in the game
    for area_dir in location_dir.read_dir()? {
        let area_dir = area_dir?;

        if !area_dir.file_type()?.is_dir() {
            continue;
        }

        // Iterate each folder in area dir, which consists of a folder for each qpid area in the area/level
        for qpid_area_dir in area_dir.path().read_dir()? {
            let qpid_area_dir = qpid_area_dir?;

            if !qpid_area_dir.file_type()?.is_dir() {
                continue;
            }

            // Iterate each .core file in qpid area dir
            for qpid_area_file in qpid_area_dir.path().read_dir()? {
                let qpid_area_file = qpid_area_file?;

                if !qpid_area_file.file_type()?.is_file() {
                    continue;
                }

                let file_path = qpid_area_file.path();

                // Check that the extension matches '.core'
                let extension = file_path.extension();

                if extension != Some(OsStr::new("core")) {
                    continue;
                }

                let relative_path = load_context.get_relative_path(&file_path)?;
                let core_file = load_context.load_file(relative_path)?.clone(); // cloning is necessary to avoid borrowing issues. there is probably a better way but i don't know it

                for delivery_point in
                    core_file.get_objects(&RTTITypeHash::DeliveryPointInfoResource)?
                {
                    let delivery_point = delivery_point
                        .as_delivery_point_info_resource()
                        .expect("Entry should always be a DeliveryPointInfoResource");

                    // Skip invalid construction types
                    match &delivery_point.delivery_point_type {
                        ConstructionPointType::DeliveryBase
                        | ConstructionPointType::PreppersShelter
                        | ConstructionPointType::StageSafetyHouse
                        | ConstructionPointType::PlayerSafetyHouse
                        | ConstructionPointType::NetSafetyHouse
                        | ConstructionPointType::StagePost
                        | ConstructionPointType::PlayerPost
                        | ConstructionPointType::NetPost
                        | ConstructionPointType::MulePost => {}
                        _ => continue,
                    }

                    let names = get_names(delivery_point, load_context)?;

                    // If there's no localization text for this area it's probably not relevant
                    if names.is_empty() {
                        continue;
                    }

                    let metadata = QpidAreaMetaData {
                        order_in_list: delivery_point.order_in_list,
                        construction_type: delivery_point.delivery_point_type,
                        area: delivery_point.area,
                        location: delivery_point.world_transform.position,
                    };

                    qpid_areas.insert(
                        delivery_point.delivery_point_locator_id,
                        QpidArea {
                            qpid_id: delivery_point.delivery_point_locator_id,
                            names,
                            metadata,
                        },
                    );
                }
            }
        }
    }

    Ok(())
}

fn get_names(
    delivery_point: &DeliveryPointInfoResource,
    load_context: &mut LoadContext,
) -> Result<BTreeMap<Language, String>, anyhow::Error> {
    let mut names = BTreeMap::new();

    if let Some(text) = delivery_point.description_text.load_resolve(load_context)? {
        let text = text
            .as_localized_text_resource()
            .expect("Text should always be LocalizedTextResource");

        for (lang, name) in &text.languages {
            names.insert(*lang, name.text.clone());
        }
    }

    Ok(names)
}
