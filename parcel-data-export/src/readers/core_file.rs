use std::{collections::HashMap, fs::File};

use binary_reader::BinaryReader;
use int_enum::IntEnum;
use uuid::Uuid;

use super::{
    baggage_list_item::BaggageListItem,
    commodity_list_item::CommodityListItem,
    delivery_point_info_resource::DeliveryPointInfoResource,
    equipment_list_item::EquipmentListItem,
    localized_text_resource::LocalizedTextResource,
    lost_baggage_with_name_and_icon_list::{
        LostBaggageWithNameAndIconListCollection, LostBaggageWithNameAndIconListResource,
    },
    raw_material_list_item::RawMaterialListItem,
    weapon_list_item::WeaponListItem,
    LoadContext, RTTIType, RTTITypeHash, Read, ReadRTTIType,
};

#[derive(Debug, Clone)]
pub struct Entry {
    pub type_hash: u64,
    pub uuid: Uuid,
    pub value: RTTIType,
}

#[derive(Debug, Clone)]
pub struct CoreFile {
    entries: Vec<Entry>,

    /// The value is the index of the entry in the `entries` vector
    uuid_lookup: HashMap<Uuid, usize>,

    /// The value is the index of the entries in the `entries` vector that have the rtti type hash of the key
    hash_lookup: HashMap<u64, Vec<usize>>,
}

impl CoreFile {
    pub fn from_file(file: &mut File, context: &mut LoadContext) -> Result<Self, anyhow::Error> {
        let mut entries = Vec::new();
        let mut reader = BinaryReader::from_file(file);
        reader.set_endian(binary_reader::Endian::Little);

        while reader.pos < reader.length {
            let hash = reader.read_u64()?;
            let len = reader.read_u32()? as usize;
            let obj_type_name = RTTITypeHash::from_int(hash)
                .map(|ty| format!("{:?}", ty))
                .unwrap_or_else(|_| format!("0x{:X}", hash));

            if reader.pos + len > reader.length {
                anyhow::bail!("Unexpected end of file");
            }

            let slice = reader.read_bytes(len)?;
            let mut slice_reader = BinaryReader::from_u8(slice);
            slice_reader.set_endian(binary_reader::Endian::Little);

            match read_object(hash, &mut slice_reader, context) {
                Ok(obj) => {
                    let uuid = *obj.object_uuid();

                    entries.push(Entry {
                        type_hash: hash,
                        uuid,
                        value: obj,
                    });
                }
                Err(err) => {
                    // Don't log unknown type hashes to avoid log spam
                    if !err.to_string().contains("Unknown RTTI type hash") {
                        println!(
                            "Could not read object {obj_type_name} from {}: {err} (offset after read: {})",
                            context.current_file_path().expect("Current file path should always be Some"),
                            (reader.pos - slice_reader.length) + slice_reader.pos
                        );
                    }
                }
            }
        }

        if reader.pos != reader.length {
            anyhow::bail!("Expected end of file");
        }

        let mut result = CoreFile {
            entries,
            uuid_lookup: HashMap::new(),
            hash_lookup: HashMap::new(),
        };
        result.update_lookups()?;
        Ok(result)
    }

    pub fn find_object<T: ReadRTTIType>(
        &self,
        uuid: &Uuid,
    ) -> Result<Option<&RTTIType>, anyhow::Error> {
        let index = self.uuid_lookup.get(uuid);

        if let Some(index) = index {
            let entry = &self.entries[*index];

            if T::rtti_type().int_value() != entry.type_hash {
                return Err(anyhow::anyhow!("Object type mismatch"));
            }

            Ok(Some(&entry.value))
        } else {
            Ok(None)
        }
    }

    pub fn get_objects(&self, ty: &RTTITypeHash) -> Result<Vec<&RTTIType>, anyhow::Error> {
        let indices = self.hash_lookup.get(&ty.int_value());

        if let Some(indices) = indices {
            let mut result = Vec::new();

            for index in indices {
                result.push(&self.entries[*index].value);
            }

            Ok(result)
        } else {
            Ok(Vec::default())
        }
    }

    fn update_lookups(&mut self) -> Result<(), anyhow::Error> {
        self.hash_lookup.clear();
        self.uuid_lookup.clear();

        for (index, entry) in self.entries.iter().enumerate() {
            if self.uuid_lookup.insert(entry.uuid, index).is_some() {
                anyhow::bail!("Duplicate UUID found");
            }

            self.hash_lookup
                .entry(entry.type_hash)
                .or_default()
                .push(index);
        }

        Ok(())
    }
}

fn read_object(
    hash: u64,
    reader: &mut BinaryReader,
    context: &mut LoadContext,
) -> Result<RTTIType, anyhow::Error> {
    match RTTITypeHash::from_int(hash) {
        Ok(RTTITypeHash::RawMaterialListItem) => {
            let item = RawMaterialListItem::read(reader, context)?;
            Ok(RTTIType::RawMaterialListItem(item))
        }
        Ok(RTTITypeHash::LocalizedTextResource) => {
            let item = LocalizedTextResource::read(reader, context)?;
            Ok(RTTIType::LocalizedTextResource(item))
        }
        Ok(RTTITypeHash::CommodityListItem) => {
            let item = CommodityListItem::read(reader, context)?;
            Ok(RTTIType::CommodityListItem(item))
        }
        Ok(RTTITypeHash::WeaponListItem) => {
            let item = WeaponListItem::read(reader, context)?;
            Ok(RTTIType::WeaponListItem(item))
        }
        Ok(RTTITypeHash::EquipmentListItem) => {
            let item = EquipmentListItem::read(reader, context)?;
            Ok(RTTIType::EquipmentListItem(item))
        }
        Ok(RTTITypeHash::BaggageListItem) => {
            let item = BaggageListItem::read(reader, context)?;
            Ok(RTTIType::BaggageListItem(item))
        }
        Ok(RTTITypeHash::DeliveryPointInfoResource) => {
            let item = DeliveryPointInfoResource::read(reader, context)?;
            Ok(RTTIType::DeliveryPointInfoResource(item))
        }
        Ok(RTTITypeHash::LostBaggageWithNameAndIconListCollection) => {
            let item = LostBaggageWithNameAndIconListCollection::read(reader, context)?;
            Ok(RTTIType::LostBaggageWithNameAndIconListCollection(item))
        }
        Ok(RTTITypeHash::LostBaggageWithNameAndIconListResource) => {
            let item = LostBaggageWithNameAndIconListResource::read(reader, context)?;
            Ok(RTTIType::LostBaggageWithNameAndIconListResource(item))
        }
        _ => anyhow::bail!("Unknown RTTI type hash"),
    }
}
