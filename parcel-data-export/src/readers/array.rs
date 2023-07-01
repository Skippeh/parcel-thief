use std::ops::{Deref, DerefMut};

use super::Read;

#[derive(Debug, Clone)]
pub struct Array<T: Read + Clone>(Vec<T>);

impl<T: Read + Clone> super::Read for Array<T> {
    fn read(
        reader: &mut binary_reader::BinaryReader,
        context: &mut super::LoadContext,
    ) -> Result<Self, anyhow::Error> {
        let len = reader.read_u32()? as usize;

        // sanity check, if we don't the length might be very large and crash the program
        if len > 10000 {
            anyhow::bail!("Something is probably wrong, reported array size = {len}");
        }

        let mut vec = Vec::with_capacity(len);

        if len > 0 {
            for _ in 0..len {
                vec.push(T::read(reader, context)?);
            }
        }

        Ok(Self(vec))
    }
}

impl<T: Read + Clone> Deref for Array<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Read + Clone> DerefMut for Array<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
