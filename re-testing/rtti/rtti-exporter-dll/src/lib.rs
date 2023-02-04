mod libc_util;
mod rtti;

use std::{
    sync::{Arc, RwLock},
    thread::spawn,
    time::Duration,
};

use lazy_static::lazy_static;
use windows::Win32::{
    Foundation::HINSTANCE,
    System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH},
};

use anyhow::Result;

lazy_static! {
    static ref WORK_FINISHED: Arc<RwLock<bool>> = Arc::new(RwLock::new(false));
}

#[no_mangle]
#[allow(non_snake_case)]
#[allow(clippy::if_same_then_else)]
pub extern "system" fn DllMain(_module: HINSTANCE, call_reason: u32, _reserved: u32) -> u32 {
    if call_reason == DLL_PROCESS_ATTACH {
        unsafe {
            match attach() {
                Ok(_) => 1,
                Err(err) => {
                    eprintln!("Failed to attach: {:?}", err);
                    0
                }
            }
        }
    } else if call_reason == DLL_PROCESS_DETACH {
        unsafe {
            match detach() {
                Ok(_) => 1,
                Err(err) => {
                    eprintln!("Failed to detach: {:?}", err);
                    0
                }
            }
        }
    } else {
        1
    }
}

unsafe fn attach() -> Result<()> {
    spawn(|| {
        if let Err(err) = rtti::export() {
            eprintln!("Failed to export data: {:?}", err);
        }

        println!("Data exported successfully");

        *WORK_FINISHED.write().unwrap() = true;
    });
    Ok(())
}

unsafe fn detach() -> Result<()> {
    loop {
        if *WORK_FINISHED.read().unwrap() {
            break;
        }

        std::thread::sleep(Duration::from_millis(100));
    }

    // exit process since the only purpose of launching it was to extract data
    std::process::exit(0);
}
