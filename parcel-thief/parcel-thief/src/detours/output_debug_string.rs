use std::{
    ffi::{CStr, OsString},
    io::Write,
    os::windows::prelude::OsStringExt,
};

use anyhow::Context;
use retour::static_detour;
use windows::{
    s, w,
    Win32::System::LibraryLoader::{GetModuleHandleW, GetProcAddress},
};

static_detour! {
    static OUTPUT_DEBUG_STRING_A_HOOK: unsafe extern "fastcall" fn(*const char); // cstr
    static OUTPUT_DEBUG_STRING_W_HOOK: unsafe extern "fastcall" fn(*const u16); // utf-16
}

pub unsafe fn hook() -> Result<(), anyhow::Error> {
    let module_handle =
        GetModuleHandleW(w!("KERNEL32")).context("Could not get base address of KERNEL32")?;

    let addr_a = GetProcAddress(module_handle, s!("OutputDebugStringA"))
        .map(|func| func as usize)
        .context("Could not get base address of OutputDebugStringA")?;

    let addr_w = GetProcAddress(module_handle, s!("OutputDebugStringW"))
        .map(|func| func as usize)
        .context("Could not get base address of OutputDebugStringW")?;

    OUTPUT_DEBUG_STRING_A_HOOK
        .initialize(std::mem::transmute(addr_a), output_debug_string_a_detour)
        .context("Could not initialize OutputDebugStringA detour")?
        .enable()
        .context("Could not enable OutputDebugStringA detour")?;

    OUTPUT_DEBUG_STRING_W_HOOK
        .initialize(std::mem::transmute(addr_w), output_debug_string_w_detour)
        .context("Could not initialize OutputDebugStringW detour")?
        .enable()
        .context("Could not enable OutputDebugStringW detour")?;

    Ok(())
}

pub unsafe fn unhook() -> Result<(), anyhow::Error> {
    OUTPUT_DEBUG_STRING_A_HOOK.disable()?;
    OUTPUT_DEBUG_STRING_W_HOOK.disable()?;

    Ok(())
}

fn output_debug_string_a_detour(c_str: *const char) {
    unsafe {
        if log::log_enabled!(log::Level::Trace) {
            let log_c_str = CStr::from_ptr(c_str as _);
            let log_str = log_c_str.to_string_lossy();
            print!("{log_str}");
            std::io::stdout().flush().ok();
        }

        OUTPUT_DEBUG_STRING_A_HOOK.call(c_str);
    }
}

fn output_debug_string_w_detour(utf16_str: *const u16) {
    unsafe {
        if log::log_enabled!(log::Level::Trace) {
            let log_os_str = u16_ptr_to_string(utf16_str);
            let log_str = log_os_str.to_string_lossy();
            print!("{log_str}");
            std::io::stdout().flush().ok();
        }

        OUTPUT_DEBUG_STRING_W_HOOK.call(utf16_str);
    }
}

unsafe fn u16_ptr_to_string(ptr: *const u16) -> OsString {
    let len = (0..).take_while(|&i| *ptr.offset(i) != 0).count();
    let slice = std::slice::from_raw_parts(ptr, len);

    OsString::from_wide(slice)
}
