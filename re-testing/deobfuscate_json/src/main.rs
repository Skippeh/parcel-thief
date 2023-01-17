mod deobfuscator;
mod strings_parser;

use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
use deobfuscator::deobfuscate_json_logs;
use strings_parser::parse_string_pairs;

#[derive(Parser)]
struct Options {
    /// The directory where the json logs are stored
    #[arg(short = 'i', long = "input")]
    logs_directory: PathBuf,
    /// The text file where all strings are stored
    #[arg(short, long)]
    strings_file: PathBuf,
    /// If true then already deobfuscated files will be deobfuscated again
    #[arg(long)]
    force: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Options::parse();

    let string_pairs = parse_string_pairs(args.strings_file.as_path())
        .await
        .context("could not parse string pairs")?;

    deobfuscate_json_logs(args.logs_directory.as_path(), &string_pairs, !args.force)
        .await
        .context("could not deobfuscate json logs")?;

    Ok(())
}
