use detour::static_detour;

static_detour! {
    static ENCRYPT_MESSAGE_HOOK: unsafe extern "fastcall" fn(*mut u8, i32, i64, *mut u8, i64);
    static DECRYPT_MESSAGE_HOOK: unsafe extern "fastcall" fn(*mut u8, i32, i64, i64, *mut *mut u8) -> i8;
}

fn encrypt_message_detour(
    p_data: *mut u8,
    data_len: i32,
    a3: i64,
    p_encrypted_data: *mut u8,
    a5: i64,
) {
    unsafe {
        let p_data_as_num = p_data as u64;
        let p_encrypted_data_as_num = p_encrypted_data as u64;
        println!("encrypt_message({p_data_as_num:#18X}, {data_len}, {a3:#18X}, {p_encrypted_data_as_num:#18X}, {a5:#18X})");

        let data_slice = std::slice::from_raw_parts(p_data as _, data_len as _);
        let data = String::from_utf8_lossy(data_slice);

        println!("data as string: {data}");

        println!();
        ENCRYPT_MESSAGE_HOOK.call(p_data, data_len, a3, p_encrypted_data, a5)
    }
}

fn decrypt_message_detour(
    p_data: *mut u8,
    data_len: i32,
    a3: i64,
    a4: i64,
    p_p_decrypted_data: *mut *mut u8,
) -> i8 {
    unsafe {
        let p_data_as_num = p_data as u64;
        let p_p_decrypted_data_as_num = p_p_decrypted_data as u64;
        println!("decrypt_message({p_data_as_num:#18X}, {data_len}, {a3:#18X}, {a4:#18X}, {p_p_decrypted_data_as_num:#18X})");

        let result = DECRYPT_MESSAGE_HOOK.call(p_data, data_len, a3, a4, p_p_decrypted_data);

        println!("returned {result}");

        let decrypted_data_slice =
            std::slice::from_raw_parts(*p_p_decrypted_data as _, data_len as _);
        let decrypted_data = String::from_utf8_lossy(decrypted_data_slice);

        println!("decrypted_data as string: {decrypted_data}");

        println!();
        result
    }
}

pub unsafe fn load() -> anyhow::Result<()> {
    println!("detours::load");

    ENCRYPT_MESSAGE_HOOK
        .initialize(
            std::mem::transmute(0x00007FF726D79290_i64),
            encrypt_message_detour,
        )?
        .enable()?;

    DECRYPT_MESSAGE_HOOK
        .initialize(
            std::mem::transmute(0x00007FF726D78F40_i64),
            decrypt_message_detour,
        )?
        .enable()?;

    Ok(())
}

pub unsafe fn unload() -> anyhow::Result<()> {
    println!("detours::unload");

    ENCRYPT_MESSAGE_HOOK.disable()?;
    DECRYPT_MESSAGE_HOOK.disable()?;

    Ok(())
}
