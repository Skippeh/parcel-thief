use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
    path::PathBuf,
    str::FromStr,
};

use uuid::Uuid;

use super::{string::DSString, LoadContext, RTTIType, Read, ReadRTTIType};

#[derive(Debug, Clone)]
pub struct Ref<T: Sized + Read + ReadRTTIType> {
    pub value: Option<Box<RTTIType>>,
    _data_type: PhantomData<T>,
}

impl<T: Sized + Read + ReadRTTIType> Ref<T> {
    pub fn new(value: Option<Box<RTTIType>>) -> Self {
        Self {
            value,
            _data_type: PhantomData,
        }
    }

    pub fn none() -> Self {
        Self {
            value: None,
            _data_type: PhantomData,
        }
    }
}

impl<T: Sized + Read + ReadRTTIType> super::Read for Ref<T> {
    fn read(
        reader: &mut binary_reader::BinaryReader,
        context: &mut LoadContext,
    ) -> Result<Self, anyhow::Error> {
        let kind = reader.read_u8()?;

        // read based on kind, but ignore if it's a link or reference because we don't need to care about that
        match kind {
            0 => Ok(Self::none()),
            // 1 = internal link, 5 = internal reference
            1 | 5 => {
                let _uuid = Uuid::from_slice_le(reader.read_bytes(16)?)?;
                anyhow::bail!("Internal reference parsing is not supported");
            }
            // 2 = external link, 3 = external reference
            2 | 3 => {
                let uuid = Uuid::from_slice_le(reader.read_bytes(16)?)?;
                let path: String = DSString::read(reader, context)?.into();

                // load file from path and resolve reference
                let file = context.load_file(&PathBuf::from_str(&path)?)?;
                let object = file
                    .find_object::<T>(&uuid)?
                    .map(|object| Box::new(object.clone()));

                Ok(Self::new(object))
            }
            _ => anyhow::bail!("Unknown reference kind: {}", kind),
        }
    }
}

impl<T: Sized + Read + ReadRTTIType> Deref for Ref<T> {
    type Target = Option<Box<RTTIType>>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Sized + Read + ReadRTTIType> DerefMut for Ref<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

#[derive(Debug, Clone)]
pub struct UnresolvedRef {
    pub uuid: Option<Uuid>,
    pub path: Option<String>,
}

impl Read for UnresolvedRef {
    fn read(
        reader: &mut binary_reader::BinaryReader,
        context: &mut LoadContext,
    ) -> Result<Self, anyhow::Error> {
        let kind = reader.read_u8()?;
        let uuid;
        let path;

        match kind {
            0 => {
                uuid = None;
                path = None;
            }
            1 | 5 => {
                uuid = Some(Uuid::from_slice_le(reader.read_bytes(16)?)?);
                path = None;
            }
            2 | 3 => {
                uuid = Some(Uuid::from_slice_le(reader.read_bytes(16)?)?);
                path = Some(DSString::read(reader, context)?.into());
            }
            _ => anyhow::bail!("Unknown reference kind: {}", kind),
        }

        Ok(Self { uuid, path })
    }
}