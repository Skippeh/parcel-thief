use std::{ffi::CStr, fmt::Debug, fs::File};

use anyhow::Context;
use binary_reader::BinaryReader;
use chrono::{DateTime, NaiveDateTime, Utc};
use ini::Ini;
use percent_encoding::percent_decode_str;

#[derive(Debug)]
pub struct SlotInfo {
    pub title: String,
    pub sub_title: String,
    pub detail: String,
    pub modification_time: DateTime<Utc>,
}

pub struct SaveFile {
    pub slot_info: SlotInfo,
    pub icon_png_data: Vec<u8>,
    pub compressed_data: Vec<u8>,
}

impl Debug for SaveFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SaveFile")
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

pub struct Reader {
    reader: BinaryReader,
}

impl Reader {
    pub fn from_file(mut file: File) -> Self {
        Self {
            reader: BinaryReader::from_file(&mut file),
        }
    }

    pub fn read_save_file(&mut self) -> Result<SaveFile, anyhow::Error> {
        let slot_info_len = self.read_str_num()?;
        let png_len = self.read_str_num()?;
        let compressed_data_len = self.read_str_num()?;

        let slot_info_bytes = self.reader.read_bytes(slot_info_len)?.to_vec();
        let icon_bytes = self.reader.read_bytes(png_len)?.to_vec();
        let compressed_data = self.reader.read_bytes(compressed_data_len)?.to_vec();

        if self.reader.pos != self.reader.length {
            anyhow::bail!("Expected to reach end of file");
        }

        let slot_info_str = String::from_utf8(slot_info_bytes)?;
        let slot_info = parse_slot_info(slot_info_str)?;

        Ok(SaveFile {
            slot_info,
            icon_png_data: icon_bytes,
            compressed_data,
        })
    }

    fn read_str_num(&mut self) -> Result<usize, anyhow::Error> {
        let ascii = self.reader.read_bytes(8)?;
        let utf8 = CStr::from_bytes_until_nul(ascii)?;
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
        .context("ModificationTime is out of range")?;
    let modification_time = DateTime::from_utc(modification_time, Utc);

    Ok(SlotInfo {
        title: read_decode!(slot, "Title"),
        sub_title: read_decode!(slot, "SubTitle"),
        detail: read_decode!(slot, "Detail"),
        modification_time,
    })
}
