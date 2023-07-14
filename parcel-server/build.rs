use std::{fs::File, path::Path, process::Command};

use parcel_common::api_types::frontend::{auth::*, baggages::*};
use typescript_type_def::{write_definition_file, DefinitionFileOptions};

fn main() {
    generate_ts_types();

    if Ok("release") == std::env::var("PROFILE").as_deref() {
        build_frontend_client();
    }
}

type Api = (
    AuthRequest,
    InitAuthResponse,
    CheckAuthRequest,
    CheckAuthResponse,
    JwtPayload,
    ListSharedCargoResponse,
);

fn generate_ts_types() {
    let mut options = DefinitionFileOptions::default();
    options.root_namespace = None;

    let file = File::create("frontend/src/api_types.ts").unwrap();
    write_definition_file::<_, Api>(file, options).unwrap();
}

fn build_frontend_client() {
    let yarn_path = which::which("yarn").expect("Could not find yarn");
    let frontend_dir = Path::new("./frontend");

    let install_result = Command::new(&yarn_path)
        .arg("install")
        .current_dir(&frontend_dir)
        .status()
        .expect("Could not run 'yarn install'");

    if !install_result.success() {
        panic!("'yarn install' did not finish successfully");
    }

    let build_result = Command::new(&yarn_path)
        .arg("build")
        .current_dir(&frontend_dir)
        .status()
        .expect("Could not run 'yarn build'");

    if !build_result.success() {
        panic!("'yarn build' did not finish successfully");
    }
}
