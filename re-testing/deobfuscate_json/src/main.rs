use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
struct Options {
    /// The directory where the json logs are stored
    #[arg(short = 'i', long = "input")]
    logs_directory: PathBuf,
    /// The directory where the deobfuscated json logs should be put
    #[arg(short = 'o', long = "output")]
    output_directory: PathBuf,
    /// The text file where all strings are stored
    #[arg(short, long)]
    strings_file: PathBuf,
}

fn main() {
    let args = Options::parse();
}
