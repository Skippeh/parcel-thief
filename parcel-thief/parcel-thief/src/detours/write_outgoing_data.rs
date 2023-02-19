use detour::static_detour;
use parcel_common::aes;

use crate::{ds_string::DsString, offsets::OFFSETS};

static_detour! {
    static WRITE_OUTGOING_DATA_HOOK: unsafe extern "fastcall" fn(i32, *const *const DsString, *mut *mut DsString);
}

pub unsafe fn hook() -> Result<(), anyhow::Error> {
    Ok(WRITE_OUTGOING_DATA_HOOK
        .initialize(
            *OFFSETS
                .read()
                .unwrap()
                .cast_mapped_offset("write_outgoing_data")
                .unwrap(),
            write_outgoing_data_detour,
        )?
        .enable()?)
}

pub unsafe fn unhook() -> Result<(), anyhow::Error> {
    Ok(WRITE_OUTGOING_DATA_HOOK.disable()?)
}

fn write_outgoing_data_detour(
    encrypt_mode: i32,
    decrypted_data: *const *const DsString,
    out_encrypted_data: *mut *mut DsString,
) {
    unsafe {
        assert_eq!(
            encrypt_mode, 2,
            "Expected encrypt_mode = 2, but it was {}",
            encrypt_mode
        );

        let decrypted_data_str = &**decrypted_data;
        let encrypted_data = DsString::from_str(&aes::encrypt_json_data(
            decrypted_data_str.as_ref().to_bytes(),
        ));
        let encrypted_data = *std::pin::pin!(encrypted_data);
        *out_encrypted_data = encrypted_data;
    }
}
