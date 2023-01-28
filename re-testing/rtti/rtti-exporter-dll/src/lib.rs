use windows::Win32::{
    Foundation::HINSTANCE,
    System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH},
};

#[no_mangle]
#[allow(non_snake_case)]
#[allow(clippy::if_same_then_else)]
pub extern "system" fn DllMain(_module: HINSTANCE, call_reason: u32, _reserved: u32) -> u32 {
    if call_reason == DLL_PROCESS_ATTACH {
        1
    } else if call_reason == DLL_PROCESS_DETACH {
        1
    } else {
        1
    }
}
