#![allow(clippy::missing_safety_doc)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use lazy_static::lazy_static;
use windows::Win32::{
    Foundation::HINSTANCE,
    System::{
        Console::{AllocConsole, FreeConsole},
        SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH},
    },
};

mod detours;

#[derive(Default)]
pub struct ParcelThief {}

impl ParcelThief {
    pub unsafe fn start(&self) -> anyhow::Result<()> {
        println!("ParcelThief::start");

        AllocConsole();

        match detours::load() {
            Ok(_) => {}
            Err(err) => {
                println!("Error loading detours: {err:?}");
                return Err(err);
            }
        }

        println!("gaming");

        Ok(())
    }

    pub unsafe fn stop(&self) -> anyhow::Result<()> {
        println!("ParcelThief::stop");

        match detours::unload() {
            Ok(_) => {}
            Err(err) => {
                println!("Error unloading detours: {err:?}");
                return Err(err);
            }
        }

        FreeConsole();

        println!("no longer gaming");

        Ok(())
    }
}

lazy_static! {
    pub static ref PARCEL_THIEF: ParcelThief = ParcelThief::default();
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn DllMain(_module: HINSTANCE, call_reason: u32, _reserved: u32) -> u32 {
    if call_reason == DLL_PROCESS_ATTACH {
        unsafe { PARCEL_THIEF.start().is_ok() as u32 }
    } else if call_reason == DLL_PROCESS_DETACH {
        unsafe { PARCEL_THIEF.stop().is_ok() as u32 }
    } else {
        1
    }
}
