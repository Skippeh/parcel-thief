use std::marker::PhantomData;

use uuid::Uuid;

use super::{string::DSString, CoreFile};

pub trait ResolveRef {
    type RefType: Sized + super::Read;

    fn resolve_ref(&self) -> Result<Self::RefType, anyhow::Error>;
}

#[derive(Debug)]
pub enum RefKind {
    None, // Means reference is unset (null)
    Link,
    Reference,
}

#[derive(Debug)]
pub enum Ref<T: Sized + super::Read> {
    Internal(InternalRef<T>),
    External(ExternalRef<T>),
}

#[derive(Debug)]
pub struct InternalRef<T: Sized + super::Read> {
    pub uuid: Uuid,
    _phantom: PhantomData<T>,
}

#[derive(Debug)]
pub struct ExternalRef<T: Sized + super::Read> {
    pub uuid: Uuid,
    pub path: String,
    _phantom: PhantomData<T>,
}

impl<T: Sized + super::Read> ResolveRef for Ref<T> {
    type RefType = T;

    fn resolve_ref(&self) -> Result<Self::RefType, anyhow::Error> {
        match self {
            Ref::Internal(rf) => rf.resolve_ref(),
            Ref::External(rf) => rf.resolve_ref(),
        }
    }
}

impl<T: Sized + super::Read> ResolveRef for InternalRef<T> {
    type RefType = T;

    fn resolve_ref(&self) -> Result<Self::RefType, anyhow::Error> {
        todo!("Internal ref resolution not implemented")
    }
}

impl<T: Sized + super::Read> ResolveRef for ExternalRef<T> {
    type RefType = T;

    fn resolve_ref(&self) -> Result<Self::RefType, anyhow::Error> {
        todo!("External ref resolution not implemented")
    }
}

impl<T: Sized + super::Read> super::Read for Option<Ref<T>> {
    fn read(reader: &mut binary_reader::BinaryReader) -> Result<Self, anyhow::Error> {
        let kind = reader.read_u8()?;

        // read based on kind, but ignore if it's a link or reference because we don't need to care about that
        match kind {
            0 => Ok(None),
            // 1 = internal link, 5 = internal reference
            1 | 5 => {
                let uuid = Uuid::from_slice_le(reader.read_bytes(16)?)?;
                Ok(Some(Ref::Internal(InternalRef {
                    uuid,
                    _phantom: PhantomData,
                })))
            }
            // 2 = external link, 3 = external reference
            2 | 3 => {
                let uuid = Uuid::from_slice_le(reader.read_bytes(16)?)?;
                let path = DSString::read(reader)?.into();
                Ok(Some(Ref::External(ExternalRef {
                    uuid,
                    path,
                    _phantom: PhantomData,
                })))
            }
            _ => anyhow::bail!("Unknown reference kind: {}", kind),
        }
    }
}
