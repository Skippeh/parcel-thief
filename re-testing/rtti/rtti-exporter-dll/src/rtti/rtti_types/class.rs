use std::borrow::Cow;

use crate::rtti::RttiBase;

//pub type SerializeToString = fn(*const u8, *mut *const u8) -> bool;

#[derive(Debug)]
#[repr(C)]
pub struct RttiClass {
    pub base: RttiBase,
    /// Number of entries in message_handler_table
    pub message_handler_count: u8,
    /// Number of entries in inherited_message_handler_table
    pub inherited_message_count: u8,
    _pad0: [u8; 0x2],
    unk0: u16,
    _pad1: [u8; 0x2],
    pub size: u32,
    pub alignment: u16,
    pub flags: u16,
    _fn_constructor: *const u8,             // function pointer
    _fn_destructor: *const u8,              // function pointer
    _fn_deserialize_from_string: *const u8, // function pointer
    pub fn_serialize_to_string: *const u8,  // function pointer
    pub name: *const char,
    _pad2: [u8; 0x18],
    pub inheritance_table: *const InheritanceEntry,
    pub member_table: *const MemberEntry,
    pub lua_function_table: *const LuaFunctionEntry,
    pub message_handler_table: *const MessageHandlerEntry,
    pub inherited_message_table: *const InheritedMessageEntry,
    _fn_get_exported_symbols: *const u8, // function pointer
}

#[derive(Debug)]
#[repr(C)]
pub struct InheritanceEntry {
    pub ty: *const RttiClass,
    pub offset: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct MemberEntry {
    pub ty: *const RttiBase,
    pub offset: u16,
    pub flags: u16,
    pub name: *const char,
    _fn_property_getter: *const u8, // function pointer
    _fn_property_setter: *const u8, // function pointer
    _pad0: [u8; 0x10],
}

#[derive(Debug)]
#[repr(C)]
pub struct LuaFunctionEntry {
    pub ret_val_type: char,
    pub name: *const char,
    pub argument_string: *const char,
    pub fn_pointer: *const u8, // function pointer
}

#[derive(Debug)]
#[repr(C)]
pub struct MessageHandlerEntry {
    pub ty: *const RttiBase,
    pub callback: *const u8, // function pointer
}

#[derive(Debug)]
#[repr(C)]
pub struct InheritedMessageEntry {
    _unk0: bool,
    pub ty: *const RttiBase,
    pub class_type: *const RttiBase,
}

impl RttiClass {
    pub unsafe fn name_to_string(&self) -> Option<Cow<str>> {
        if self.name as usize == 0 {
            return None;
        }

        let len = libc::strlen(self.name as *const i8);
        Some(String::from_utf8_lossy(std::slice::from_raw_parts(
            self.name as *const u8,
            len,
        )))
    }
}
