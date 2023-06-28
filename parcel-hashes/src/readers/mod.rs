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
    core_file::CoreFile, localized_text_resource::LocalizedTextResource,
    raw_material_list_item::RawMaterialListItem,
};

pub mod core_file;
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
    LocalizedTextResource = 0x31BE502435317445,
}

#[derive(Debug, Clone, enum_as_inner::EnumAsInner)]
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
pub struct LoadContext {
    base_directory: PathBuf,
    files: HashMap<String, CoreFile>,
}

impl LoadContext {
    pub fn new(data_directory: PathBuf) -> Self {
        Self {
            base_directory: data_directory,
            files: HashMap::new(),
        }
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

        let file_path = self.base_directory.join(path);
        let mut file = File::open(file_path.clone())
            .with_context(|| format!("Loading file: {file_path:?}"))?;
        let file = CoreFile::from_file(&mut file, self)?;

        self.files.insert(path_str.clone(), file);

        let result = self
            .files
            .get(&path_str)
            .expect("File should always be found");

        Ok(result)
    }
}
