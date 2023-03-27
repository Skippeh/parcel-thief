mod checkpoint_reader;
mod oodle;
mod profile_reader;
mod save_file_reader;

use std::{
    fs::File,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

use anyhow::Context;
use clap::Parser;
use save_file_reader::SaveFileReader;

#[derive(Parser)]
struct Options {
    pub save_path: PathBuf,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Options::parse();

    if !args.save_path.exists() {
        anyhow::bail!("Save file not found: \"{}\"", args.save_path.display());
    }

    if !args.save_path.is_file() {
        anyhow::bail!("Save path does not point to a file");
    }

    let reader = SaveFileReader::from_file(File::open(&args.save_path)?);
    let save_file = reader
        .read_save_file()
        .context("Could not read save file")?;

    let mut save_directory = args.save_path;
    save_directory.pop();

    match save_file {
        save_file_reader::SaveFile::Checkpoint(data) => {
            dbg!(&data.slot_info);

            let checkpoint_data = dbg!(checkpoint_reader::read_compressed_data(
                data.compressed_data
            )?);

            let mut png_path = save_directory.clone();
            png_path.push("icon.png");
            write_file(&png_path, &data.icon_png_data)?;

            let mut decompressed_data_path = save_directory.clone();
            decompressed_data_path.push("decompressed-data.bin");
            write_file(&decompressed_data_path, &checkpoint_data.data)?;
        }
        save_file_reader::SaveFile::Profile(data) => {
            let profile_data = dbg!(profile_reader::read_compressed_data(data.compressed_data)?);

            let mut decompressed_data_path = save_directory.clone();
            decompressed_data_path.push("decompressed-data.bin");
            write_file(&decompressed_data_path, &profile_data.data)?;
        }
    }

    Ok(())
}

fn write_file(path: &Path, data: &[u8]) -> Result<(), anyhow::Error> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    writer.write_all(data)?;

    Ok(())
}
