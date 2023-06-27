#[derive(Debug)]
pub struct RTTIObject;

impl super::Read for RTTIObject {
    fn read(_: &mut binary_reader::BinaryReader) -> Result<Self, anyhow::Error> {
        Ok(Self)
    }
}
