#![allow(clippy::missing_safety_doc)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::sync::{Arc, RwLock};

use anyhow::Context;
use clap::Parser;
use http::{
    uri::{Parts, PathAndQuery},
    Uri,
};
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameVersion {
    Steam,
    Epic,
}

lazy_static! {
    static ref PARCEL_THIEF: ParcelThief = ParcelThief::default();
    pub static ref GAME_VERSION: Arc<RwLock<GameVersion>> =
        Arc::new(RwLock::new(GameVersion::Steam));
    pub static ref SERVER_AUTH_URL: Arc<RwLock<String>> =
        Arc::new(RwLock::new("http://localhost/auth/ds".into()));
    pub static ref LAUNCH_OPTIONS: Arc<RwLock<LaunchOptions>> =
        Arc::new(RwLock::new(LaunchOptions::default()));
}

// Note: it's important that there are no required values here since the game is most likely
// started without any arguments specified.
// It's also important that each arg has a "parcel" prefix to distinguish it from other args.
#[derive(Default, Parser)]
pub struct LaunchOptions {
    #[arg(long = "parcel-server-url")]
}

#[derive(Default)]
pub struct ParcelThief {}

impl ParcelThief {
    pub unsafe fn start(&self) -> anyhow::Result<()> {
        AllocConsole();
        println!("ParcelThief::start");

        match LaunchOptions::try_parse() {
            Ok(opts) => *LAUNCH_OPTIONS.write().unwrap() = opts,
            Err(err) => anyhow::bail!(err.to_string()),

        if let Some(url) = load_server_url().context("Could not load or parse server url")? {
            println!("Using server url: {}", url);
            *SERVER_AUTH_URL.write().unwrap() = url.to_string();
        } else {
            const err_message: &str = "\
Could not find server url, make sure at least one of these exist:
  - --parcel-server-url launch parameter
  - parcel-server-url.txt in game directory
  - PARCEL_SERVER_URL environment variable\
            ";

            println!("{}", err_message);
            let _ = msgbox::create("Error", err_message, msgbox::IconType::Error);

            anyhow::bail!("Server url not found");
        }

        *GAME_VERSION.write().unwrap() =
            find_game_version().context("Could not figure out game version")?;

        println!("Detected game version: {:?}", GAME_VERSION.read().unwrap());

        offsets::map_offsets().context("Failed to map offsets")?;

        println!("Mapped offsets");

        detours::load().context("Could not load detours")?;

        {
            let server_url = SERVER_AUTH_URL.read().unwrap();
            println!("Setting auth url: {}", server_url);
            auth::load(&server_url);
        }

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

unsafe fn load_server_url() -> Result<Option<String>, anyhow::Error> {
    // First check if --parcel-server-url launch parameter is defined
    let launch_opts = LAUNCH_OPTIONS.read().unwrap();

    let url: Result<_, anyhow::Error> = (|| {
        if let Some(server_url) = &launch_opts.server_url {
            return Ok(Some(server_url.clone()));
        }

        // Secondly, check if parcel-server-url.txt exists and load server url from it
        if let Ok(server_url) = std::fs::read_to_string("parcel-server-url.txt") {
            return Ok(Some(
                server_url
                    .parse::<Uri>()
                    .context("Could not parse from parcel-server-url.txt")?,
            ));
        }

        // Lastly, check if the env variable is set
        if let Ok(server_url) = std::env::var("PARCEL_SERVER_URL") {
            return Ok(Some(
                server_url
                    .parse::<Uri>()
                    .context("Could not parse from environment variable")?,
            ));
        }

        Ok(None)
    })();

    match url {
        Ok(Some(mut uri)) => {
            if uri.scheme().is_none() || uri.authority().is_none() {
                anyhow::bail!("Missing scheme or authority. The url format should be similar to http://example.com or https://ds.example.com for example");
            }

            let create_default_auth_path = |uri: &Uri| {
                Uri::builder()
                    .authority(uri.authority().expect("Missing authority").clone())
                    .scheme(uri.scheme().expect("Missing scheme").clone())
                    .path_and_query(PathAndQuery::from_static("/auth/ds"))
                    .build()
                    .expect("Uri should always be valid")
            };

            match uri.path_and_query() {
                Some(path_and_query) => {
                    if path_and_query == "/" {
                        uri = create_default_auth_path(&uri);
                    }
                }
                None => uri = create_default_auth_path(&uri),
            };

            Ok(Some(uri.to_string().trim_end_matches('/').to_owned()))
        }
        Ok(None) => Ok(None),
        Err(err) => Err(err),
    }
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
                    let _ = msgbox::create(
                        "Injection failed",
                        &format!("{:?}", err),
                        msgbox::IconType::Error,
                    );
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
                    let _ = msgbox::create(
                        "Ejection failed",
                        &format!("{:?}", err),
                        msgbox::IconType::Error,
                    );
                    0
                }
            }
        }
    } else {
        1
    }
}
