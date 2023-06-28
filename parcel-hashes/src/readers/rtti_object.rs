use super::LoadContext;

#[derive(Debug, Clone)]
pub struct RTTIObject;

impl super::Read for RTTIObject {
    fn read(
        _: &mut binary_reader::BinaryReader,
        _: &mut LoadContext,
    ) -> Result<Self, anyhow::Error> {
        Ok(Self)
    }
}
