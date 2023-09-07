use std::{cmp::min, ffi::CStr, fmt::Debug, fs::File};

use anyhow::Context;
use binary_reader::BinaryReader;
use chrono::{DateTime, NaiveDateTime, Utc};
use ini::Ini;
use murmurhash3::murmurhash3_x64_128;
use percent_encoding::percent_decode_str;

#[derive(Debug)]
pub struct SlotInfo {
    pub title: String,
    pub sub_title: String,
    pub detail: String,
    pub user_param: String,
    pub modification_time: DateTime<Utc>,
}

#[derive(Debug)]
pub enum SaveFile {
    Checkpoint(CompressedCheckpointData),
    Profile(CompressedProfileData),
}

pub struct CompressedCheckpointData {
    pub slot_info: SlotInfo,
    pub icon_png_data: Vec<u8>,
    pub compressed_data: Vec<u8>,
}

impl Debug for CompressedCheckpointData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SaveFile")
            .field("slot_info", &self.slot_info)
            .field(
                "icon_png_data",
                &format_args!("Vec<u8> ({} bytes)", self.icon_png_data.len()),
            )
            .field(
                "compressed_data",
                &format_args!("Vec<u8> ({} bytes)", self.compressed_data.len()),
            )
            .finish()
    }
}

pub struct CompressedProfileData {
    pub compressed_data: Vec<u8>,
}

impl Debug for CompressedProfileData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProfileData")
            .field(
                "compressed_data",
                &format_args!("Vec<u8> ({} bytes)", self.compressed_data.len()),
            )
            .finish()
    }
}

pub struct SaveFileReader {
    reader: BinaryReader,
}

impl SaveFileReader {
    pub fn from_file(mut file: File) -> Self {
        let mut reader = BinaryReader::from_file(&mut file);
        reader.set_endian(binary_reader::Endian::Little);
        Self { reader }
    }

    pub fn read_save_file(mut self) -> Result<SaveFile, anyhow::Error> {
        let test_read = self.read_str_num();

        match test_read {
            Ok(slot_info_len) => {
                println!("Detected checkpoint save file");

                let png_len = self.read_str_num()?;
                let compressed_data_len = self.read_str_num()?;

                let slot_info_bytes = self.reader.read_bytes(slot_info_len)?.to_vec();
                let icon_bytes = self.reader.read_bytes(png_len)?.to_vec();
                let mut compressed_data = self.reader.read_bytes(compressed_data_len)?.to_vec();

                if self.reader.pos != self.reader.length {
                    anyhow::bail!("Expected to reach end of file");
                }

                let slot_info_str = String::from_utf8(slot_info_bytes)?;
                let slot_info = parse_slot_info(slot_info_str)?;
                decrypt_compressed_data(&mut compressed_data)?;

                Ok(SaveFile::Checkpoint(CompressedCheckpointData {
                    slot_info,
                    icon_png_data: icon_bytes,
                    compressed_data,
                }))
            }
            Err(_) => {
                self.reader.pos = 0;
                println!("Detected profile save file");
                let mut data = self.reader.read_bytes(self.reader.length)?.to_vec();
                decrypt_compressed_data(&mut data)?;

                Ok(SaveFile::Profile(CompressedProfileData {
                    compressed_data: data,
                }))
            }
        }
    }

    fn read_str_num(&mut self) -> Result<usize, anyhow::Error> {
        let ascii = self.reader.read_bytes(8)?;
        let utf8 = CStr::from_bytes_until_nul(ascii)
            .with_context(|| format!("Could not read c_string from {:?}", ascii))?;
        Ok(utf8.to_str()?.parse()?)
    }
}

macro_rules! read_decode {
    ($slot:ident, $name:literal) => {
        percent_decode_str($slot.get($name).context(format!("{} not found", $name))?)
            .decode_utf8_lossy()
            .into()
    };
}

fn parse_slot_info(ini_string: String) -> Result<SlotInfo, anyhow::Error> {
    let ini = Ini::load_from_str(&ini_string)?;
    let slot = ini
        .section(Some("Slot"))
        .context("Slot section not found")?;

    let mut modification_time = slot
        .get("ModificationTime")
        .context("ModificationTime not found")?
        .parse::<i64>()?;

    // convert time to epoch in milliseconds
    modification_time -= 62135596800000000; // epoch expressed in microseconds
    modification_time /= 1000;

    let modification_time = NaiveDateTime::from_timestamp_millis(modification_time)
        .context("ModificationTime is out of range")?
        .and_utc();

    Ok(SlotInfo {
        title: read_decode!(slot, "Title"),
        sub_title: read_decode!(slot, "SubTitle"),
        detail: read_decode!(slot, "Detail"),
        user_param: read_decode!(slot, "UserParam"),
        modification_time,
    })
}

const STATIC_XOR_KEY: [u8; 16] = [
    0x46, 0x3F, 0xBB, 0xBC, 0x7B, 0x6F, 0x57, 0xED, 0x9B, 0x41, 0x7D, 0x36, 0xE3, 0x26, 0x12, 0xBC,
];

fn decrypt_compressed_data(data: &mut Vec<u8>) -> Result<(), anyhow::Error> {
    let data_format = u32::from_le_bytes(
        data.get(..4)
            .context("Compressed data is less than 4 bytes")?
            .try_into()?,
    );

    const FORMAT_SAVEFILE: u32 = 0xC0391E1D;
    const FORMAT_UNK: u32 = 0xC0391E55;

    if data_format == FORMAT_SAVEFILE || data_format == FORMAT_UNK {
        let file_key = data
            .get(4..8)
            .context("Compressed data is less than 8 bytes")?;

        let mut xor_key = vec![0; 16];
        xor_key[0..4].copy_from_slice(file_key);
        xor_key[4..16].copy_from_slice(&STATIC_XOR_KEY[4..16]);

        // Remove file format type and file hash salt from source
        data.drain(0..8);

        let mut hash = vec![0u8; 16];
        let (hash_1, hash_2) = murmurhash3_x64_128(&xor_key, 42);
        hash[0..8].copy_from_slice(&hash_1.to_le_bytes());
        hash[8..16].copy_from_slice(&hash_2.to_le_bytes());

        for chunk in data.chunks_mut(16) {
            for i in 0..min(16, chunk.len()) {
                chunk[i] ^= hash[i];
            }
        }
    }

    Ok(())
}
