use crate::rtti::RttiBase;

#[derive(Debug)]
#[repr(C)]
pub struct RttiPrimitive {
    pub base: RttiBase,
    _pad0: [u8; 0x8],
    pub name: *const char,
    pub parent_type: *const RttiBase,

    // function pointers
    _fn_deserialize_string: *const u8,
    _fn_serialize_string: *const u8,
    _fn_assign_value: *const u8,
    _fn_test_equality: *const u8,
    _fn_constructor: *const u8,
    _fn_destructor: *const u8,
    _fn_swap_endianness: *const u8,
    _fn_try_assign_value: *const u8,
    _fn_get_size_in_memory: *const u8,
    _fn_compare_by_strings: *const u8,
    _fn_unknown_function: *const u8,
}
