use std::{
    mem,
    sync::{Arc, Mutex},
};

use lazy_static::lazy_static;
use libc::c_void;
use windows::{w, Win32::System::LibraryLoader::GetModuleHandleW};

lazy_static! {
    static ref AUTH_URL_ADDR: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
}

const ORIGINAL_AUTH_URL_ADDR: usize = 0x4DF8108;

pub unsafe fn load() -> anyhow::Result<()> {
    println!("detours::load");

    set_auth_url(Some("http://localhost/auth/ds"));

    Ok(())
}

pub unsafe fn unload() -> anyhow::Result<()> {
    println!("detours::unload");

    set_auth_url(None); // restore original auth url

    Ok(())
}

unsafe fn set_auth_url(url: Option<&str>) {
    // free last url if it's set
    let mut current_auth_url_ptr = AUTH_URL_ADDR.lock().unwrap();

    if *current_auth_url_ptr != 0 {
        libc::free(*current_auth_url_ptr as *mut c_void);
        *current_auth_url_ptr = 0;
    }

    let base_addr = get_base_addr().unwrap();
    let auth_url_ptr = (base_addr + 0x4DF8130) as *mut *mut u8;

    if let Some(url) = url {
        // size = (string length as u32) * 2 + string length + terminator
        let url_ptr = libc::malloc(8 + url.len() + 1) as *mut u8;
        *current_auth_url_ptr = url_ptr as usize;

        // write len as u32 twice to the first 8 bytes
        // not sure why the game does this, but it do
        *(url_ptr as *mut u32) = url.len() as u32;
        *(url_ptr.add(4) as *mut u32) = url.len() as u32;

        // write characters at +8 offset from ptr
        url.as_ptr().copy_to(url_ptr.add(8), url.len());
        *(url_ptr.add(8 + url.len())) = 0u8;

        *auth_url_ptr = url_ptr.add(8);
    } else {
        *auth_url_ptr = (base_addr + ORIGINAL_AUTH_URL_ADDR) as *mut u8;
    }
}

unsafe fn get_base_addr() -> anyhow::Result<usize> {
    let base_addr = GetModuleHandleW(w!("ds.exe"))?;
    Ok(mem::transmute(base_addr))
}
