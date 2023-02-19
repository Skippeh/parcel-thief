mod read_incoming_data;
mod write_outgoing_data;

use windows::{w, Win32::System::LibraryLoader::GetModuleHandleW};

pub unsafe fn load() -> anyhow::Result<()> {
    println!("detours::load");

    read_incoming_data::hook()?;
    write_outgoing_data::hook()?;

    set_auth_url("http://localhost/auth/ds");

    Ok(())
}

pub unsafe fn unload() -> anyhow::Result<()> {
    println!("detours::unload");

    read_incoming_data::unhook()?;
    write_outgoing_data::unhook()?;

    set_auth_url("https://prod-pc-15.wws-gs2.com/ds/auth"); // restore original auth url

    Ok(())
}

unsafe fn set_auth_url(url: &str) {
    let base_addr = get_base_addr().unwrap();
    let auth_url_ptr = (base_addr + 0x4DF8130) as *mut *mut u8;

    // size = (string length as u32) * 2 + string length + terminator
    let url_ptr = libc::malloc(8 + url.len() + 1) as *mut u8;

    // write len as u32 twice to the first 8 bytes
    // not sure why the game does this, but it do
    *(url_ptr as *mut u32) = url.len() as u32;
    *(url_ptr.add(4) as *mut u32) = url.len() as u32;

    // write characters at +8 offset from ptr
    url.as_ptr().copy_to(url_ptr.add(8), url.len());
    *(url_ptr.add(8 + url.len())) = 0u8;

    *auth_url_ptr = url_ptr.add(8);
}

unsafe fn get_base_addr() -> anyhow::Result<usize> {
    let base_addr = GetModuleHandleW(w!("ds.exe"))?;
    Ok(std::mem::transmute(base_addr))
}
