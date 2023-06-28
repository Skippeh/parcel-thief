use std::{collections::HashMap, fs::File, ops::Deref, result};

use binary_reader::BinaryReader;
use int_enum::IntEnum;
use uuid::Uuid;

use self::{
    localized_text_resource::LocalizedTextResource, raw_material_list_item::RawMaterialListItem,
    rtti_object::RTTIObject, rtti_ref_object::RTTIRefObject,
};

pub mod core_object;
pub mod game_list_item_base;
pub mod game_list_item_base_with_icon;
pub mod localized_text_resource;
pub mod raw_material_list_item;
pub mod reference;
pub mod resource;
pub mod rtti_object;
pub mod rtti_ref_object;
pub mod string;

#[derive(Debug)]
#[repr(u8)]
pub enum ListItemColor {
    Red,
    Yellow,
    Blue,
    Gray,
    Orange,
    Purple,
}

impl Read for ListItemColor {
    fn read(reader: &mut BinaryReader) -> Result<Self, anyhow::Error> {
        Ok(match reader.read_u8()? {
            0 => ListItemColor::Red,
            1 => ListItemColor::Yellow,
            2 => ListItemColor::Blue,
            3 => ListItemColor::Gray,
            4 => ListItemColor::Orange,
            5 => ListItemColor::Purple,
            other => anyhow::bail!("Unknown color variant: {other}"),
        })
    }
}

pub trait ReadRTTIType {
    fn rtti_type() -> RTTITypeHash;
}

pub trait Read: Sized {
    fn read(reader: &mut BinaryReader) -> Result<Self, anyhow::Error>;
}

#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntEnum)]
pub enum RTTITypeHash {
    RawMaterialListItem = 0x6543AE76010E714E,
    LocalizedTextResource = 0x31BE502435317445,
}

#[derive(Debug, enum_as_inner::EnumAsInner)]
pub enum RTTIType {
    RawMaterialListItem(RawMaterialListItem),
    LocalizedTextResource(LocalizedTextResource),
}

impl RTTIType {
    pub fn object_uuid(&self) -> &Uuid {
        match self {
            // this is stupid but i don't care enough to refactor it since this project is relatively small and is only for development purposes
            RTTIType::RawMaterialListItem(item) => {
                &item
                    .as_ref()
                    .as_ref()
                    .as_ref()
                    .as_ref()
                    .as_ref()
                    .object_uuid
            }
            RTTIType::LocalizedTextResource(item) => &item.as_ref().as_ref().as_ref().object_uuid,
        }
    }
}

#[derive(Debug)]
pub struct Entry {
    pub type_hash: u64,
    pub uuid: Uuid,
    pub value: RTTIType,
}

#[derive(Debug)]
pub struct CoreFile {
    entries: Vec<Entry>,

    /// The value is the index of the entry in the `entries` vector
    uuid_lookup: HashMap<Uuid, usize>,

    /// The value is the index of the entries in the `entries` vector that have the rtti type hash of the key
    hash_lookup: HashMap<u64, Vec<usize>>,
}

impl CoreFile {
    pub fn from_file(file: &mut File) -> Result<Self, anyhow::Error> {
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

            println!(
                "Reading {} at offset {} of {} byte(s)",
                obj_type_name,
                reader.pos - slice_reader.length,
                slice_reader.length
            );

            match read_object(hash, &mut slice_reader) {
                Ok(obj) => {
                    if slice_reader.pos != slice_reader.length {
                        anyhow::bail!("Did not read all bytes ({}/{})", slice_reader.pos, slice_reader.length);
                    }

                    let uuid = *obj.object_uuid();

                    entries.push(Entry {
                        type_hash: hash,
                        uuid,
                        value: obj,
                    });
                }
                Err(err) => {
                    println!(
                        "Could not read object {obj_type_name}: {err} (offset after read: {})",
                        (reader.pos - slice_reader.length) + slice_reader.pos
                    );
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

    pub fn find_object<T: ReadRTTIType>(&self, uuid: &Uuid) -> Result<Option<&RTTIType>, anyhow::Error> {
        let index = self.uuid_lookup.get(uuid);

        if let Some(index) = index {
            let entry = &self.entries[*index];

            if T::rtti_type().int_value() != entry.type_hash {
                return Err(anyhow::anyhow!("Object type mismatch"))
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

            self.hash_lookup.entry(entry.type_hash).or_default().push(index);
        }

        Ok(())
    }
}

fn read_object(hash: u64, reader: &mut BinaryReader) -> Result<RTTIType, anyhow::Error> {
    match RTTITypeHash::from_int(hash) {
        Ok(RTTITypeHash::RawMaterialListItem) => {
            let item = RawMaterialListItem::read(reader)?;
            Ok(RTTIType::RawMaterialListItem(item))
        }
        Ok(RTTITypeHash::LocalizedTextResource) => {
            let item = LocalizedTextResource::read(reader)?;
            Ok(RTTIType::LocalizedTextResource(item))
        }
        _ => anyhow::bail!("Unknown RTTI type hash"),
    }
}
