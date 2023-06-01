mod watcher;

use std::{
    path::{Path, PathBuf},
    time::Duration,
};

use anyhow::Context;
use clap::Parser;
use crossterm::event::{Event, EventStream, KeyEventKind};
use dll_syringe::{
    process::{BorrowedProcess, OwnedProcess, Process},
    Syringe,
};
use futures::StreamExt;
use tokio::{select, time::sleep};
use watcher::FileWatcher;

#[derive(Parser)]
struct Options {
    #[clap(default_value = "parcel_client.dll")]
    dll_path: PathBuf,
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

            let syringe = Syringe::for_process(target_process);

            let mut copied_dll_path = args.dll_path.clone();
            let mut file_name = copied_dll_path.file_name().unwrap().to_owned();
            file_name.push("_temp");
            copied_dll_path.pop();
            copied_dll_path.push(file_name);

            std::fs::copy(&args.dll_path, &copied_dll_path).context("Failed to copy target dll")?;

            let injected_payload = match syringe.inject(&copied_dll_path) {
                Ok(payload) => payload,
                Err(err) => {
                    println!("Failed to inject: {err:?}");
                    break;
                }
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
