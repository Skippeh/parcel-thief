pub unsafe fn read_cstring<T>(addr: *const T) -> String {
    let str_len = libc::strlen(addr as *const i8);
    let str_slice = std::slice::from_raw_parts(addr as *const u8, str_len);
    let name_str = String::from_utf8_lossy(str_slice);
    name_str.into_owned()
}
