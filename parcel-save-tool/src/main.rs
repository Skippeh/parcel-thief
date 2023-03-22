mod reader;

use std::{
    fs::File,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

use clap::Parser;
use reader::Reader;

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

    let mut reader = Reader::from_file(File::open(&args.save_path)?);
    let save_file = reader.read_save_file()?;

    dbg!(&save_file.slot_info);

    let mut save_directory = args.save_path;
    save_directory.pop();

    let mut png_path = save_directory.clone();
    png_path.push("icon.png");

    let mut compressed_data_path = save_directory.clone();
    compressed_data_path.push("compressed-data.bin");

    write_file(&png_path, &save_file.icon_png_data)?;
    write_file(&compressed_data_path, &save_file.compressed_data)?;

    Ok(())
}

fn write_file(path: &Path, data: &[u8]) -> Result<(), anyhow::Error> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    writer.write_all(data)?;

    Ok(())
}
