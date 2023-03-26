use std::io::Write;

use detour::static_detour;

use crate::offsets::OFFSETS;

static_detour! {
    static OODLE_DECOMPRESS_HOOK: unsafe extern "fastcall" fn(
        *const u8,
        i64,
        *mut u8,
        i64,
        i32,
        i32,
        i32,
        *const usize,
        *const usize,
        *const usize,
        *const usize,
        *const usize,
        i64,
        i32
    ) -> i64;

    static OODLE_COMPRESS_HOOK: unsafe extern "fastcall" fn(
        i32,
        *const u8,
        i64,
        *mut u8,
        i32,
        *const usize,
        *const usize,
        *const usize,
        *const usize,
        i64
    )-> i64;
}

pub unsafe fn hook() -> Result<(), anyhow::Error> {
    OODLE_DECOMPRESS_HOOK
        .initialize(
            *OFFSETS
                .read()
                .unwrap()
                .cast_mapped_offset(crate::offsets::LocationOffset::FnOodleDecompress),
            oodle_decompress_detour,
        )?
        .enable()?;

    OODLE_COMPRESS_HOOK
        .initialize(
            *OFFSETS
                .read()
                .unwrap()
                .cast_mapped_offset(crate::offsets::LocationOffset::FnOodleCompress),
            oodle_compress_detour,
        )?
        .enable()?;

    Ok(())
}

pub unsafe fn unhook() -> Result<(), anyhow::Error> {
    OODLE_DECOMPRESS_HOOK.disable()?;
    OODLE_COMPRESS_HOOK.disable()?;

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn oodle_decompress_detour(
    src: *const u8,
    mut src_len: i64,
    dest: *mut u8,
    dest_len: i64,
    fuzz: i32,
    crc: i32,
    verbose: i32,
    context: *const usize,
    e: *const usize,
    callback: *const usize,
    callback_ctx: *const usize,
    scratch: *const usize,
    scratch_size: i64,
    thread_phase: i32,
) -> i64 {
    if src_len == 153909 {
        src_len -= 23;
    }

    unsafe {
        let dest_len = OODLE_DECOMPRESS_HOOK.call(
            src,
            src_len,
            dest,
            dest_len,
            fuzz,
            crc,
            verbose,
            context,
            e,
            callback,
            callback_ctx,
            scratch,
            scratch_size,
            thread_phase,
        );

        if src_len == 153909 - 23 {
            println!("compressed len: {}", src_len);
            println!("decompress len: {}", dest_len);

            let src_slice = &*std::ptr::slice_from_raw_parts(src, src_len as usize);
            let dest_slice = &*std::ptr::slice_from_raw_parts(dest, dest_len as usize);

            fn write_file(name: &str, bytes: &[u8]) {
                let mut file = std::fs::File::create(name).expect("create file successfully");
                file.write_all(bytes).expect("write bytes successfully");
                file.flush().expect("flush successfully");

                println!("wrote {} bytes to {}", bytes.len(), name);
            }

            write_file("decompress_src.bin", src_slice);
            write_file("decompress_dest.bin", dest_slice);
        }

        dest_len
    }
}

#[allow(clippy::too_many_arguments)]
fn oodle_compress_detour(
    compressor: i32,
    src: *const u8,
    src_len: i64,
    dest: *mut u8,
    level: i32,
    options: *const usize,
    unk6: *const usize,
    unused: *const usize,
    scratch: *const usize,
    scratch_size: i64,
) -> i64 {
    unsafe {
        let dest_len = OODLE_COMPRESS_HOOK.call(
            compressor,
            src,
            src_len,
            dest,
            level,
            options,
            unk6,
            unused,
            scratch,
            scratch_size,
        );

        println!("compressed len: {}", src_len);
        println!("decompress len: {}", dest_len);

        let src_slice = &*std::ptr::slice_from_raw_parts(src, src_len as usize);
        let dest_slice = &*std::ptr::slice_from_raw_parts(dest, dest_len as usize);

        fn write_file(name: &str, bytes: &[u8]) {
            let mut file = std::fs::File::create(name).expect("create file successfully");
            file.write_all(bytes).expect("write bytes successfully");
            file.flush().expect("flush successfully");

            println!("wrote {} bytes to {}", bytes.len(), name);
        }

        write_file("compress_src.bin", src_slice);
        write_file("compress_dest.bin", dest_slice);

        dest_len
    }
}
