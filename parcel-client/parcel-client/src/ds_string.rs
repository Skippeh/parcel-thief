use std::{ffi::CStr, marker::PhantomData};

use crate::offsets::{LocationOffset, OFFSETS};

#[repr(C)]
struct InternalData {
    _ref_count: u32,
    _crc: u32,
    len: u32,
    _unk0: u32,
}

/// # Arguments
/// * `0`: string_out pointer
/// * `1`: pointer to null terminated c string
type StringCtor = unsafe extern "fastcall" fn(*mut *mut DsString, *const u8) -> u8;

type StringDtor = unsafe extern "fastcall" fn(*const *const DsString);

#[repr(C)]
pub struct DsString {
    phantom: PhantomData<u8>, // prevents other modules from allocating this struct manually
}

impl DsString {
    pub unsafe fn len(&self) -> usize {
        self.internal_data().len as usize
    }

    unsafe fn internal_data(&self) -> &InternalData {
        let self_ptr = std::mem::transmute::<_, *const u8>(self as *const Self);
        &*self_ptr.sub(16).cast::<InternalData>()
    }

    pub unsafe fn from_str(val: &str) -> *mut Self {
        let ctor_fn = OFFSETS
            .read()
            .unwrap()
            .cast_mapped_offset::<StringCtor>(LocationOffset::FnStringCtor);

        let mut val_bytes_with_nul = vec![0u8; val.len() + 1];
        if !val.is_empty() {
            val_bytes_with_nul[0..val.len()].copy_from_slice(val.as_bytes());
        }

        let cstr_val = CStr::from_bytes_with_nul(&val_bytes_with_nul).unwrap();
        let string_pos = &*std::pin::pin!(0usize) as *const usize as _;

        ctor_fn(string_pos, cstr_val.as_ptr() as *const u8);

        *string_pos
    }

    #[allow(dead_code)]
    pub unsafe fn dtor(ds_str: &'static Self) {
        let dtor_fn = OFFSETS
            .read()
            .unwrap()
            .cast_mapped_offset::<StringDtor>(LocationOffset::FnStringDtor);

        let addr = &*std::pin::pin!(ds_str as *const DsString) as _;
        dtor_fn(addr);
    }
}

impl ToString for DsString {
    fn to_string(&self) -> String {
        self.as_ref().to_string_lossy().into_owned()
    }
}

impl AsRef<CStr> for DsString {
    fn as_ref(&self) -> &CStr {
        unsafe {
            let ptr = std::mem::transmute::<_, *const u8>(self as *const DsString);
            let len = self.len();
            let bytes = &*std::ptr::slice_from_raw_parts(ptr, len + 1);

            CStr::from_bytes_with_nul(bytes).expect("string should be null terminated")
        }
    }
}
