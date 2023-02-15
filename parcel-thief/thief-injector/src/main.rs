use std::{path::PathBuf, time::Duration};

use clap::Parser;
use crossterm::event::{Event, EventStream, KeyEventKind};
use dll_syringe::{
    process::{BorrowedProcess, OwnedProcess, Process},
    Syringe,
};
use futures::StreamExt;
use tokio::{select, time::sleep};

#[derive(Parser)]
struct Options {
    #[clap(default_value = "parcel-thief.dll")]
    dll_path: PathBuf,
}

#[tokio::main]
async fn main() {
    let args = Options::parse();

    if !args.dll_path.exists() {
        println!("Dll file does not exist: {}", args.dll_path.display());
        return;
    }

    if !args.dll_path.is_file() {
        println!("Dll path is not a file: {}", args.dll_path.display());
        return;
    }

    println!("Press any key to eject and exit at anytime.");

    loop {
        let target_process = OwnedProcess::find_first_by_name("ds");

        if let Some(target_process) = target_process {
            println!("Found process, injecting...");

            let syringe = Syringe::for_process(target_process);

            let injected_payload = match syringe.inject(&args.dll_path) {
                Ok(payload) => payload,
                Err(err) => {
                    println!("Failed to inject: {err:?}");
                    break;
                }
            };

            println!("Successfully injected");

            select! {
                _ = wait_for_next_key() => {}
                _ = poll_process_exit(syringe.process()) => {
                    println!("Process died, waiting again for ds...");
                    sleep(Duration::from_secs(1)).await; // wait a bit for process to exit completely
                    continue;
                }
            };

            println!("Ejecting...");

            match syringe.eject(injected_payload) {
                Ok(_) => println!("Ejected successfully"),
                Err(err) => {
                    println!("Failed to eject: {err:?}");
                }
            };

            break;
        } else {
            select! {
                _ = wait_for_next_key() => { break; }
                _ = sleep(Duration::from_millis(100)) => {}
            }
        }
    }
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
