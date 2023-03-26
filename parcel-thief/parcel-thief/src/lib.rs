#![allow(clippy::missing_safety_doc)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::sync::{Arc, RwLock};

use anyhow::Context;
use lazy_static::lazy_static;
use windows::{
    w,
    Win32::{
        Foundation::HINSTANCE,
        System::{
            Console::{AllocConsole, FreeConsole},
            LibraryLoader::GetModuleHandleW,
            SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH},
        },
    },
};

mod auth;
mod detours;
mod ds_string;
mod offsets;
mod pattern;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameVersion {
    Steam,
    Epic,
}

lazy_static! {
    pub static ref GAME_VERSION: Arc<RwLock<GameVersion>> =
        Arc::new(RwLock::new(GameVersion::Steam));
}

#[derive(Default)]
pub struct ParcelThief {}

impl ParcelThief {
    pub unsafe fn start(&self) -> anyhow::Result<()> {
        AllocConsole();
        println!("ParcelThief::start");

        *GAME_VERSION.write().unwrap() =
            find_game_version().context("Could not figure out game version")?;

        println!("Detected game version: {:?}", GAME_VERSION.read().unwrap());

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

unsafe fn find_game_version() -> Result<GameVersion, anyhow::Error> {
    let working_dir = std::env::current_dir()?;
    let mut steam_dll_path = working_dir.clone();
    steam_dll_path.push("steam_api64.dll");

    let mut egs_dll_path = working_dir;
    egs_dll_path.push("EOSSDK-Win64-Shipping.dll");

    if steam_dll_path.exists() {
        Ok(GameVersion::Steam)
    } else if egs_dll_path.exists() {
        Ok(GameVersion::Epic)
    } else {
        anyhow::bail!(
            "Could not find steam_api64.dll or EOSSDK-Win64-Shipping.dll in working directory"
        );
    }
}

lazy_static! {
    static ref PARCEL_THIEF: ParcelThief = ParcelThief::default();
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn DllMain(_module: HINSTANCE, call_reason: u32, _reserved: u32) -> u32 {
    if call_reason == DLL_PROCESS_ATTACH {
        unsafe {
            match PARCEL_THIEF.start() {
                Ok(_) => 1,
                Err(err) => {
                    println!("Did not attach successfully: {:?}", err);
                    0
                }
            }
        }
    } else if call_reason == DLL_PROCESS_DETACH {
        unsafe {
            match PARCEL_THIEF.stop() {
                Ok(_) => 1,
                Err(err) => {
                    println!("Did not detach successfully: {:?}", err);
                    0
                }
            }
        }
    } else {
        1
    }
}
