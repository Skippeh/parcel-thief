use std::borrow::Cow;

use crate::rtti::RttiBase;

#[derive(Debug)]
#[repr(C)]
pub struct Data {
    pub name: *const char,
}

impl Data {
    pub unsafe fn name_to_string(&self) -> Cow<str> {
        let len = libc::strlen(self.name as *const i8);
        String::from_utf8_lossy(std::slice::from_raw_parts(self.name as *const u8, len))
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct RttiContainer {
    pub base: RttiBase,
    pub ty: *const RttiBase,
    pub data: *const Data,
}
