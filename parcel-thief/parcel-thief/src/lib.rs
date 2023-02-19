#![allow(clippy::missing_safety_doc)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use anyhow::Context;
use lazy_static::lazy_static;
use windows::Win32::{
    Foundation::HINSTANCE,
    System::{
        Console::{AllocConsole, FreeConsole},
        SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH},
    },
};

mod auth;
mod detours;
mod ds_string;
mod offsets;
mod pattern;

#[derive(Default)]
pub struct ParcelThief {}

impl ParcelThief {
    pub unsafe fn start(&self) -> anyhow::Result<()> {
        AllocConsole();
        println!("ParcelThief::start");

        offsets::map_offsets().context("Failed to map offsets")?;

        println!("mapped offsets");

        detours::load().context("Could not load detours")?;

        println!("setting auth url");

        auth::load();

        println!("gaming");

        Ok(())
    }

    pub unsafe fn stop(&self) -> anyhow::Result<()> {
        println!("ParcelThief::stop");

        detours::unload().context("Could not unload detours")?;
        auth::unload();

        println!("no longer gaming");

        FreeConsole();

        Ok(())
    }
}

lazy_static! {
    static ref PARCEL_THIEF: ParcelThief = ParcelThief::default();
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
