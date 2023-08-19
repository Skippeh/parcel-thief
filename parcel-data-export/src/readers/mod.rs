use std::{
    collections::HashMap,
    fs::File,
    path::{Path, PathBuf},
};

use anyhow::Context;
use binary_reader::BinaryReader;
use int_enum::IntEnum;
use uuid::Uuid;

use self::{
    baggage_list_item::BaggageListItem, commodity_list_item::CommodityListItem,
    core_file::CoreFile, delivery_point_info_resource::DeliveryPointInfoResource,
    equipment_list_item::EquipmentListItem, localized_text_resource::LocalizedTextResource,
    raw_material_list_item::RawMaterialListItem, weapon_list_item::WeaponListItem,
};

pub mod array;
pub mod baggage_list_item;
pub mod commodity_list_item;
pub mod core_file;
pub mod core_object;
pub mod delivery_point_info_resource;
pub mod equipment_list_item;
pub mod game_list_item_base;
pub mod game_list_item_base_with_icon;
pub mod localized_text_resource;
pub mod mission_abstract_point_resource;
mod mission_static_abstract_point_resource;
pub mod raw_material_list_item;
pub mod reference;
pub mod resource;
pub mod rtti_object;
pub mod rtti_ref_object;
pub mod string;
pub mod weapon_list_item;

pub trait ReadRTTIType {
    fn rtti_type() -> RTTITypeHash;
}

pub trait Read: Sized {
    fn read(reader: &mut BinaryReader, context: &mut LoadContext) -> Result<Self, anyhow::Error>;
}

#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntEnum)]
pub enum RTTITypeHash {
    RawMaterialListItem = 0x6543AE76010E714E,
    CommodityListItem = 0x59441CF90AC3CF1B,
    WeaponListItem = 0xED0E1221E8D4D3A3,
    EquipmentListItem = 0xA6078EBE103EDA4C,
    BaggageListItem = 0x72CB5ED4F1C815EE,
    LocalizedTextResource = 0x31BE502435317445,
    DeliveryPointInfoResource = 0x202B5F4B6410D206,
}

#[derive(Debug, Clone, enum_as_inner::EnumAsInner)]
pub enum RTTIType {
    RawMaterialListItem(RawMaterialListItem),
    CommodityListItem(CommodityListItem),
    WeaponListItem(WeaponListItem),
    EquipmentListItem(EquipmentListItem),
    BaggageListItem(BaggageListItem),
    LocalizedTextResource(LocalizedTextResource),
    DeliveryPointInfoResource(DeliveryPointInfoResource),
}

impl RTTIType {
    pub fn object_uuid(&self) -> &Uuid {
        match self {
            RTTIType::RawMaterialListItem(item) => &item.object_uuid,
            RTTIType::CommodityListItem(item) => &item.object_uuid,
            RTTIType::WeaponListItem(item) => &item.object_uuid,
            RTTIType::EquipmentListItem(item) => &item.object_uuid,
            RTTIType::BaggageListItem(item) => &item.object_uuid,
            RTTIType::LocalizedTextResource(item) => &item.object_uuid,
            RTTIType::DeliveryPointInfoResource(item) => &item.object_uuid,
        }
    }
}
pub struct LoadContext {
    base_directory: PathBuf,
    files: HashMap<String, CoreFile>,
    load_queue: Vec<String>,
}

impl LoadContext {
    pub fn new(data_directory: PathBuf) -> Self {
        Self {
            base_directory: data_directory,
            files: HashMap::new(),
            load_queue: Vec::new(),
        }
    }

    pub fn get_absolute_path(&self, path: &Path) -> PathBuf {
        self.base_directory.join(path)
    }

    pub fn get_relative_path<'a>(
        &self,
        path: &'a Path,
    ) -> Result<&'a Path, std::path::StripPrefixError> {
        path.strip_prefix(&self.base_directory)
    }

    /// Returns the relative file path to the current file being loaded.
    /// The path always includes the '.core' extension.
    pub fn current_file_path(&self) -> Option<&String> {
        self.load_queue.last()
    }

    pub fn load_file(&mut self, path: &Path) -> Result<&CoreFile, anyhow::Error> {
        let mut path = path.to_owned();

        if path.extension().is_none() {
            path = path.with_extension("core");
        }

        let path_str = path.to_string_lossy().into_owned();

        // check if file is already loaded
        if self.files.contains_key(&path_str) {
            return Ok(self
                .files
                .get(&path_str)
                .expect("File should always be found"));
        }

        self.load_queue.push(path_str.clone());

        let load_result = || -> anyhow::Result<_> {
            let file_path = self.base_directory.join(path);
            let mut file = File::open(file_path.clone())
                .with_context(|| format!("Loading file: {file_path:?}"))?;
            let file = CoreFile::from_file(&mut file, self)?;

            self.files.insert(path_str.clone(), file);
            Ok(())
        }();

        self.load_queue.pop(); // always pop current file regardless of success

        load_result?;

        let result = self
            .files
            .get(&path_str)
            .expect("File should always be found");

        Ok(result)
    }

    pub fn get_file(&self, path: &Path) -> Option<&CoreFile> {
        let mut path = path.to_owned();

        if path.extension().is_none() {
            path = path.with_extension("core");
        }

        let path_str = path.to_string_lossy().into_owned();

        self.files.get(&path_str)
    }
}
