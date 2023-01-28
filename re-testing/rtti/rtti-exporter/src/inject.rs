use std::path::Path;

use anyhow::{Context, Result};
use dll_syringe::{process::OwnedProcess, Syringe};

pub fn inject_process_and_export(process_id: u32, dll_path: &Path) -> Result<()> {
    let process = OwnedProcess::from_pid(process_id).context("Could not find process by id")?;
    let syringe = Syringe::for_process(process);
    let module = syringe.inject(dll_path)?;

    if let Err(err) = syringe.eject(module) {
        eprintln!(
            "Warning: Could not eject dll (probably doesn't matter): {}",
            err
        );
    }

    Ok(())
}
