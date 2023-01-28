mod inject;

use std::{
    fs::canonicalize,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
struct Options {
    /// The filepath to ds.exe
    pub exe_path: PathBuf,
    /// The filepath to rtti_exporter_dll.dll
    #[arg(default_value = "rtti_exporter_dll.dll")]
    pub dll_path: PathBuf,
}

fn main() -> Result<()> {
    let args = Options::parse();

    validate_path_leads_to_file_name(args.exe_path.as_path(), "ds.exe")?;
    validate_path_leads_to_file_name(args.dll_path.as_path(), "rtti_exporter_dll.dll")?;

    if !args.dll_path.exists() {
        anyhow::bail!(
            "target injection dll not found at path: {}",
            &args.dll_path.display()
        );
    }

    let mut exe_directory = args.exe_path.clone();
    exe_directory.pop();
    exe_directory = canonicalize(exe_directory)?;
    let mut process = Command::new(args.exe_path)
        .current_dir(exe_directory)
        .spawn()
        .context("could not start ds.exe")?;

    let export_result = inject::inject_process_and_export(process.id(), args.dll_path.as_path());

    if let Err(err) = &export_result {
        eprintln!("Could not export rtti data: {}", err);
    }

    if let Err(err) = process.kill() {
        eprintln!("Could not kill process: {}", err);
    };

    Ok(())
}

fn validate_path_leads_to_file_name(file_path: &Path, file_name: &str) -> Result<()> {
    match file_path.file_name() {
        None => anyhow::bail!("Provided path does not lead to {}", file_name),
        Some(path) => {
            if !path.eq_ignore_ascii_case(file_name) {
                anyhow::bail!("Provided path does not lead to {}", file_name);
            }

            Ok(())
        }
    }
}
