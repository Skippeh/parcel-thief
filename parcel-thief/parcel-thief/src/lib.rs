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

use crate::offsets::OFFSETS;

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

        map_offsets().context("Failed to map offsets")?;

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

fn map_offsets() -> Result<(), anyhow::Error> {
    let offsets = &mut OFFSETS.write().unwrap();

    offsets
        .map_pattern_offset(
            "String::ctor",
            "40 53 48 83 EC 20 48 8B D9 48 C7 01 00 00 00 00 49 C7 C0 FF FF FF FF",
        )
        .context("Failed to find String::ctor offset")?;

    offsets
        .map_pattern_offset(
            "String::dtor",
            "40 53 48 83 EC 20 48 8B 19 48 8D 05 ? ? ? ? 48 83 EB 10",
        )
        .context("Failed to find String::dtor offset")?;

    offsets
        .map_pattern_offset(
            "read_incoming_data",
            "48 89 5C 24 08 55 56 57 41 56 41 57 48 83 EC 50 48 8B 05 D9 50 58 03 48 33 C4 48 89",
        )
        .context("Failed to find read_incoming_data offset")?;
    offsets
        .map_pattern_offset(
            "write_outgoing_data",
            "48 89 5C 24 08 48 89 74 24 20 55 57 41 56 48 8B EC 48 81 EC 80 00 00 00 48 8B 05 11",
        )
        .context("Failed to find write_outgoing_data offset")?;

    offsets.map_offset("auth_url", 0x4DF8130)?;

    Ok(())
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
