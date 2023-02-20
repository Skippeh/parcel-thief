use detour::static_detour;
use parcel_common::aes;

use crate::{
    ds_string::DsString,
    offsets::{LocationOffset, OFFSETS},
};

static_detour! {
    static READ_INCOMING_DATA_HOOK: unsafe extern "fastcall" fn(i32, *const *const DsString, *mut *mut DsString) -> u8;
}

pub unsafe fn hook() -> Result<(), anyhow::Error> {
    Ok(READ_INCOMING_DATA_HOOK
        .initialize(
            *OFFSETS
                .read()
                .unwrap()
                .cast_mapped_offset(LocationOffset::FnReadIncomingData),
            read_incoming_data_detour,
        )?
        .enable()?)
}

pub unsafe fn unhook() -> Result<(), anyhow::Error> {
    Ok(READ_INCOMING_DATA_HOOK.disable()?)
}

fn read_incoming_data_detour(
    decrypt_mode: i32,
    encrypted_data: *const *const DsString,
    out_decrypted_data: *mut *mut DsString,
) -> u8 {
    unsafe {
        assert_eq!(
            decrypt_mode, 2,
            "Expected decrypt_mode = 2, but it was {}",
            decrypt_mode
        );

        let encrypted_data_str = &**encrypted_data;

        match aes::decrypt_json_data(encrypted_data_str.as_ref().to_str().unwrap()) {
            Ok(data) => {
                let str = *std::pin::pin!(DsString::from_str(&data));
                *out_decrypted_data = str;
                1
            }
            Err(_) => 0,
        }
    }
}
