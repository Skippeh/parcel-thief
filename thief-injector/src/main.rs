use std::io::BufRead;

use dll_syringe::{process::OwnedProcess, Syringe};

fn main() {
    static PROCESS_NAME: &str = "ds";
    static DLL_PATH: &str = "target/x86_64-pc-windows-msvc/release/parcel_thief.dll";

    println!("waiting for {PROCESS_NAME}");

    loop {
        let target_process = match OwnedProcess::find_first_by_name(PROCESS_NAME) {
            Some(target) => target,
            None => {
                std::thread::sleep(std::time::Duration::from_millis(100));
                continue;
            }
        };

        println!("found process, injecting");

        let syringe = Syringe::for_process(target_process);

        let injected_payload = match syringe.inject(DLL_PATH) {
            Ok(payload) => payload,
            Err(err) => {
                println!("failed to inject {err:?}");
                continue;
            }
        };

        println!("injected! press enter to eject");

        let stdin = std::io::stdin();
        let mut lines_iter = stdin.lock().lines();
        let _ = lines_iter.next();

        println!("ejecting");

        match syringe.eject(injected_payload) {
            Ok(_) => println!("ejected successfully"),
            Err(err) => println!("failed to eject {err:?}"),
        };

        break;
    }
}
