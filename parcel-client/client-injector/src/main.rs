mod watcher;

use std::{
    path::{Path, PathBuf},
    str::FromStr,
    time::Duration,
};

use anyhow::Context;
use clap::{Parser, ValueEnum};
use crossterm::event::{Event, EventStream, KeyEventKind};
use dll_syringe::{
    process::{self, BorrowedProcess, OwnedProcess, Process},
    Syringe,
};
use futures::StreamExt;
use tokio::{select, time::sleep};
use watcher::FileWatcher;

#[derive(Debug, ValueEnum, Clone, Copy, PartialEq, Eq)]
enum LaunchMode {
    /// Launches the game from Steam or Epic Games Launcher (depending on which files are found in the working directory) and injects the dll
    Auto,
    /// Launches the game from Steam and injects the dll
    Steam,
    /// Launches the game from Epic Games Launcher and injects the dll
    Epic,
    /// Watches for the game process and injects the dll when found, and also reinjects if the dll is changed or the game is restarted.
    /// The dll is not reinjected if an injection/ejection error occurs
    Watch,
}

#[derive(Parser)]
struct Options {
    #[clap(default_value = "parcel_client.dll")]
    dll_path: PathBuf,
    #[clap(long("mode"), short('m'), default_value = "auto")]
    launch_mode: LaunchMode,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Options::parse();

    if !args.dll_path.exists() {
        println!("Dll file does not exist: {}", args.dll_path.display());
        return Ok(());
    }

    if !args.dll_path.is_file() {
        println!("Dll path is not a file: {}", args.dll_path.display());
        return Ok(());
    }

    match args.launch_mode {
        LaunchMode::Watch => watch(&args).await?,
        mode => launch(&args, mode).await?,
    }

    Ok(())
}

async fn watch(args: &Options) -> Result<(), anyhow::Error> {
    println!("Press any key to eject and exit at any time. The dll will automatically be re-injected when a change is detected.");
    println!("Waiting for ds.exe...");

    loop {
        let mut target_process = OwnedProcess::find_first_by_name("ds");

        if target_process.is_none() {
            // epic exe is named differently than steam
            target_process = OwnedProcess::find_first_by_name("DeathStranding");
        }

        if let Some(target_process) = target_process {
            println!("Found process, injecting...");

            let mut copied_dll_path = args.dll_path.clone();
            let mut file_name = copied_dll_path.file_name().unwrap().to_owned();
            file_name.push("_temp");
            copied_dll_path.pop();
            copied_dll_path.push(file_name);

            std::fs::copy(&args.dll_path, &copied_dll_path).context("Failed to copy target dll")?;

            let syringe = Syringe::for_process(target_process);
            let injected_payload = match inject_dll(&syringe, &copied_dll_path) {
                Some(value) => value,
                None => break,
            };

            println!("Successfully injected");

            let mut should_exit = false;
            let mut should_eject = false;
            select! {
                _ = wait_for_next_key() => {
                    should_eject = true;
                    should_exit = true;
                }
                _ = wait_for_file_change(&args.dll_path) => {
                    println!("File change detected, reloading dll");
                    should_eject = true;
                }
                _ = poll_process_exit(syringe.process()) => {
                    println!("Process died, waiting again for ds...");
                    sleep(Duration::from_secs(1)).await; // wait a bit for process to exit completely
                }
            };

            if should_eject {
                println!("Ejecting...");

                match syringe.eject(injected_payload) {
                    Ok(_) => println!("Ejected successfully"),
                    Err(err) => {
                        println!("Failed to eject: {err:?}");
                    }
                };
            }

            if should_eject || should_exit {
                std::fs::remove_file(&copied_dll_path).context("Failed to delete temporary dll")?;
            }

            if should_exit {
                break;
            }
        } else {
            // Sleep for 100 ms or until a key is pressed and exit if the latter happens
            select! {
                _ = wait_for_next_key() => { break; }
                _ = sleep(Duration::from_millis(100)) => {}
            }
        }
    }

    Ok(())
}

async fn launch(args: &Options, mode: LaunchMode) -> Result<(), anyhow::Error> {
    let (mode, exe_path) = match &mode {
        LaunchMode::Epic => (
            LaunchMode::Epic,
            PathBuf::from_str("DeathStranding.exe").expect("Path should always be valid"),
        ),
        LaunchMode::Steam => (
            LaunchMode::Steam,
            PathBuf::from_str("ds.exe").expect("Path should always be valid"),
        ),
        LaunchMode::Auto => {
            let mode;
            // check if ds.exe exists
            let mut path = PathBuf::from_str("ds.exe").expect("Path should always be valid");

            // if not, check if DeathStranding.exe exists
            if !path.exists() {
                mode = LaunchMode::Epic;
                path =
                    PathBuf::from_str("DeathStranding.exe").expect("Path should always be valid");
            } else {
                mode = LaunchMode::Steam;
            }

            (mode, path)
        }
        LaunchMode::Watch => unreachable!(),
    };

    if args.launch_mode == LaunchMode::Auto && !exe_path.exists() {
        anyhow::bail!("Could not find game executable. Make sure you're running this program from the game directory, or specify mode manually with --mode.");
    }

    let launch_uri = match mode {
        LaunchMode::Epic => {
            "com.epicgames.launcher://apps/c38f51843fdf4db0b24fc907b8d78221?action=launch&silent=true"
        }
        LaunchMode::Steam => {
            "steam://rungameid/1850570"
        }
        _ => unreachable!(),
    };

    open::that(launch_uri).context("Failed to launch game")?;

    println!("Waiting for game to launch...");

    let mut try_count = 0;
    let process;
    loop {
        let process_name = exe_path
            .with_extension("")
            .file_name()
            .expect("Path should always have a file name")
            .to_str()
            .expect("Path should always be valid")
            .to_owned();
        if let Some(found_process) = OwnedProcess::find_first_by_name(process_name) {
            process = found_process;
            break;
        } else {
            try_count += 1;

            // Search for process 20 times before giving up
            if try_count >= 20 {
                anyhow::bail!("Failed to find game process");
            }

            // Sleep for a second before trying again
            sleep(Duration::from_secs(1)).await;
        }
    }

    let syringe = Syringe::for_process(process);
    inject_dll(&syringe, &args.dll_path);

    Ok(())
}

fn inject_dll<'a>(
    syringe: &'a Syringe,
    dll_path: &PathBuf,
) -> Option<process::ProcessModule<BorrowedProcess<'a>>> {
    let injected_payload = match syringe.inject(dll_path) {
        Ok(payload) => payload,
        Err(err) => {
            println!("Failed to inject: {err:?}");
            return None;
        }
    };
    Some(injected_payload)
}

async fn poll_process_exit(process: BorrowedProcess<'_>) {
    loop {
        if process.is_alive() {
            sleep(Duration::from_millis(100)).await;
        } else {
            break;
        }
    }
}

async fn wait_for_next_key() {
    let mut reader = EventStream::new();

    loop {
        let event = reader.next().await;

        if let Some(Ok(Event::Key(key))) = event {
            if key.kind == KeyEventKind::Press {
                break;
            }
        }
    }
}

async fn wait_for_file_change(dll_path: &Path) -> Result<(), anyhow::Error> {
    let mut watcher = FileWatcher::new()?;
    watcher.watch(dll_path, notify::RecursiveMode::NonRecursive)?;

    loop {
        let event = watcher.next().await;

        match event {
            Ok(event) => match &event.kind {
                notify::EventKind::Create(_) | notify::EventKind::Modify(_) => {
                    println!("reload");
                    break;
                }
                _ => {}
            },
            Err(err) => return Err(err.into()),
        }
    }

    Ok(())
}
