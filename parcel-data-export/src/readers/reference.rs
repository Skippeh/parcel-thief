use std::{marker::PhantomData, ops::Deref, path::PathBuf};

use uuid::Uuid;

use super::{string::DSString, LoadContext, RTTIType, Read, ReadRTTIType};

#[derive(Debug, Clone)]
pub struct Ref<T: Sized + Read + ReadRTTIType> {
    inner: Option<(Uuid, PathBuf)>,
    _data_type: PhantomData<T>,
}

impl<T: Sized + Read + ReadRTTIType> Ref<T> {
    pub fn new(file_path: PathBuf, uuid: Uuid) -> Self {
        Self {
            inner: Some((uuid, file_path)),
            _data_type: PhantomData,
        }
    }

    pub fn none() -> Self {
        Self {
            inner: None,
            _data_type: PhantomData,
        }
    }

    /// Loads the referenced data if it's not loaded and returns it. Requires mutable access to the load context.
    ///
    /// Fails if the referenced file fails to load or if the referenced object type does not match the type of the reference.
    pub fn load_resolve<'loader>(
        &self,
        load_context: &'loader mut LoadContext,
    ) -> Result<Option<&'loader RTTIType>, anyhow::Error> {
        match self.inner.as_ref() {
            Some((uuid, file_path)) => {
                // load file from path and resolve reference
                let file = load_context.load_file(file_path)?;
                let object = file.find_object::<T>(&uuid)?;

                Ok(object)
            }
            None => Ok(None),
        }
    }

    /// Resolves the reference if its loaded, otherwise returns None. Does not require mutable access to the load context.
    ///
    /// Fails if the referenced object type does not match the type of the reference.
    pub fn resolve<'loader>(
        &self,
        load_context: &'loader LoadContext,
    ) -> Result<Option<&'loader RTTIType>, anyhow::Error> {
        match self.inner.as_ref() {
            Some((uuid, file_path)) => match load_context.get_file(file_path) {
                Some(file) => {
                    let object = file.find_object::<T>(&uuid)?;

                    Ok(object)
                }
                None => Ok(None),
            },
            None => Ok(None),
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
                let uuid = Uuid::from_slice_le(reader.read_bytes(16)?)?;

                Ok(Self::new(
                    context
                        .current_file_path()
                        .expect("Current file path should always be Some")
                        .try_into()?,
                    uuid,
                ))
            }
            // 2 = external link, 3 = external reference
            2 | 3 => {
                let uuid = Uuid::from_slice_le(reader.read_bytes(16)?)?;
                let path = DSString::read(reader, context)?.deref().try_into()?;

                Ok(Self::new(path, uuid))
            }
            _ => anyhow::bail!("Unknown reference kind: {}", kind),
        }
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
