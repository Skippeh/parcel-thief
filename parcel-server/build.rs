use std::fs::File;

use parcel_common::api_types::frontend::{auth::*, baggages::*};
use typescript_type_def::{write_definition_file, DefinitionFileOptions};

fn main() {
    generate_ts_types();
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
