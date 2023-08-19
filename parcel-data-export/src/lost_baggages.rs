use std::{collections::BTreeMap, path::Path};

use anyhow::Context;

use crate::readers::{LoadContext, RTTITypeHash};

pub fn read_lost_baggages(
    load_context: &mut LoadContext,
    result: &mut BTreeMap<i32, Vec<u32>>,
) -> Result<(), anyhow::Error> {
    let file = load_context
        .load_file(Path::new(
            "ds/mission/system/lost_baggage/lost_baggage_name_and_icon_list.core",
        ))?
        .clone();

    for lost_baggage_collection in
        file.get_objects(&RTTITypeHash::LostBaggageWithNameAndIconListCollection)?
    {
        let collection = lost_baggage_collection
            .as_lost_baggage_with_name_and_icon_list_collection()
            .expect("Entry should always be a LostBaggageWithNameAndIconListCollection");

        for baggages_resource_ref in collection.list.iter() {
            let baggages_resource = baggages_resource_ref
                .load_resolve(load_context)?
                .context("Ref should never be None")?
                .as_lost_baggage_with_name_and_icon_list_resource()
                .context("Ref should always be a LostBaggageWithNameAndIconListResource")?
                .clone(); // clone required to avoid borrowing issues

            let destination_qpid_id = baggages_resource
                .destination
                .load_resolve(load_context)?
                .context("Ref should never be None")?
                .as_delivery_point_info_resource()
                .context("Ref should always be a DeliveryPointInfoResource")?
                .delivery_point_locator_id;

            let mut name_hashes = Vec::with_capacity(baggages_resource.baggages.len());

            // resolve baggage refs and read item name hash
            for baggage_ref in baggages_resource.baggages.iter() {
                let baggage = baggage_ref
                    .load_resolve(load_context)?
                    .context("Ref should never be None")?
                    .as_baggage_list_item()
                    .context("Ref should always be a BaggageListItem")?;

                name_hashes.push(baggage.name_code);
            }

            result.insert(destination_qpid_id, name_hashes);
        }
    }

    Ok(())
}
