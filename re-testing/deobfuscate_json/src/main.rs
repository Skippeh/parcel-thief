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
    /// The directory where the deobfuscated json logs should be put.
    /// If unspecified then deobfuscated files will be put next to the original files (as a copy).
    #[arg(short = 'o', long = "output")]
    output_directory: Option<PathBuf>,
    /// The text file where all strings are stored
    #[arg(short, long)]
    strings_file: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Options::parse();

    let string_pairs = parse_string_pairs(args.strings_file.as_path())
        .await
        .context("could not parse string pairs")?;

    deobfuscate_json_logs(
        args.logs_directory.as_path(),
        args.output_directory
            .as_ref()
            .unwrap_or(&args.logs_directory)
            .as_path(),
        &string_pairs,
    )
    .await
    .context("could not deobfuscate json logs")?;

    Ok(())
}
