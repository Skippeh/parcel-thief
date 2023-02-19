use crate::{ds_string::DsString, offsets::OFFSETS};

pub unsafe fn load() {
    set_auth_url("http://localhost/auth/ds");
}

pub unsafe fn unload() {
    set_auth_url("https://prod-pc-15.wws-gs2.com/ds/auth"); // restore original auth url
}

unsafe fn set_auth_url(url: &str) {
    let auth_url_ptr = *OFFSETS
        .read()
        .unwrap()
        .cast_mapped_offset::<*mut *mut DsString>("auth_url")
        .unwrap();

    // Technically this creates a memory leak since the string is never free'd.
    // Realistically however this function is only called once, so complicating things doesn't do any favors.
    *auth_url_ptr = DsString::from_str(url);
}
