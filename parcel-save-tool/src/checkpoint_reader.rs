use std::fmt::Debug;

use binary_reader::BinaryReader;

use crate::oodle;

/// As far as i know the uncompressed size for checkpoint files is always this. The unused allocated space in the file is filled with 0.
const FIXED_CHECKPOINT_DATA_LENGTH: usize = 0xC00000;

pub struct CheckpointData {
    pub account_id: String,
    pub data: Vec<u8>,
}

impl Debug for CheckpointData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CheckpointData")
            .field("account_id", &self.account_id)
            .field(
                "data",
                &format_args!("Vec<u8> (bytes: {})", self.data.len()),
            )
            .finish()
    }
}

pub fn read_compressed_data(data: Vec<u8>) -> Result<CheckpointData, anyhow::Error> {
    let mut reader = BinaryReader::from_vec(&data);
    reader.set_endian(binary_reader::Endian::Little);

    let account_id_len = reader.read_u32()? as usize;
    let account_id = String::from_utf8(reader.read_bytes(account_id_len)?.to_vec())?;

    let rest_of_data = reader.read_bytes(reader.length - reader.pos)?;
    let data = oodle::decompress(rest_of_data, FIXED_CHECKPOINT_DATA_LENGTH, true, false)?;

    Ok(CheckpointData { account_id, data })
}
