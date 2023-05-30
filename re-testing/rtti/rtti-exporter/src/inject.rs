use std::{
    fs::canonicalize,
    io::{BufRead, BufReader},
    path::Path,
    process::{Command, Stdio},
    thread::spawn,
    time::Duration,
};

use anyhow::{Context, Result};
use dll_syringe::{process::OwnedProcess, Syringe};

#[allow(clippy::lines_filter_map_ok)]
pub fn inject_process_and_export(exe_path: &Path, dll_path: &Path) -> Result<()> {
    let mut exe_directory = exe_path.to_owned();
    exe_directory.pop();
    exe_directory = canonicalize(exe_directory)?;
    let mut child_process = Command::new(exe_path)
        .current_dir(exe_directory)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("could not start ds.exe")?;

    let process =
        OwnedProcess::from_pid(child_process.id()).context("Could not find process by id")?;
    let syringe = Syringe::for_process(process);
    let module = syringe.inject(dll_path)?;

    // Continuously read stdout and stderr and write to this processes stdout/stderr
    let stdout_reader = BufReader::new(child_process.stdout.take().unwrap());
    spawn(move || {
        stdout_reader
            .lines()
            .filter_map(|line| line.ok())
            .for_each(|line| println!("{}", line));
    });

    let stderr_reader = BufReader::new(child_process.stderr.take().unwrap());
    spawn(move || {
        stderr_reader
            .lines()
            .filter_map(|line| line.ok())
            .for_each(|line| eprintln!("{}", line));
    });

    let _ = syringe.eject(module); // ignore result because the process terminates on eject after waiting for work to finish

    std::thread::sleep(Duration::from_millis(500)); // Give the process some time to exit properly

    Ok(())
}
