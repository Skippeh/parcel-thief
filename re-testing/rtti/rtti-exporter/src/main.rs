mod inject;

use std::path::{Path, PathBuf};

use anyhow::Result;
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

    if let Err(err) = inject::inject_process_and_export(&args.exe_path, &args.dll_path) {
        eprintln!("Could not export rtti data: {}", err);
    }

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
