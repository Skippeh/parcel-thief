use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
struct Options {
    pub exe_path: PathBuf,
}

fn main() {
    let args = Options::parse();
}
