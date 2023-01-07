use std::{ffi::c_void, mem::transmute};

use windows::{
    core::{IUnknown, GUID, HRESULT, HSTRING, PCWSTR},
    s,
    Win32::{
        Foundation::{BOOL, FARPROC, HINSTANCE, MAX_PATH},
        System::{
            Console::AllocConsole,
            LibraryLoader::{GetProcAddress, LoadLibraryW},
            SystemInformation::GetSystemDirectoryW,
            SystemServices::DLL_PROCESS_ATTACH,
        },
    },
};

static mut ORIGINAL_DIRECT_INPUT8_CREATE: FARPROC = None;

#[no_mangle]
pub extern "system" fn DirectInput8Create(
    inst: HINSTANCE,
    version: u32,
    riidltf: *const GUID,
    out: *const *const c_void,
    unk_outer: *const IUnknown,
) -> HRESULT {
    type Func = extern "system" fn(
        inst: HINSTANCE,
        version: u32,
        riidltf: *const GUID,
        out: *const *const c_void,
        unk_outer: *const IUnknown,
    ) -> HRESULT;

    let func: Func = unsafe { transmute(ORIGINAL_DIRECT_INPUT8_CREATE) };
    func(inst, version, riidltf, out, unk_outer)
}

pub fn setup_dinput8_hook() {
    let system_directory = unsafe {
        let mut buf = [0u16; MAX_PATH as usize];
        GetSystemDirectoryW(Some(&mut buf));
        PCWSTR::from_raw(buf.as_mut_ptr()).to_string().unwrap()
    };
    let dll_path = format!("{}\\dinput8.dll", system_directory);
    let dll_instance = unsafe { LoadLibraryW(PCWSTR::from(&HSTRING::from(dll_path))) }.unwrap();

    if dll_instance.is_invalid() {
        panic!();
    }
    let func = unsafe { GetProcAddress(dll_instance, s!("DirectInput8Create")) };
    unsafe { ORIGINAL_DIRECT_INPUT8_CREATE = Some(func.unwrap()) };
}

#[no_mangle]
pub extern "system" fn DllMain(
    _inst_dll: HINSTANCE,
    reason: u32,
    _reserved: *const c_void,
) -> BOOL {
    if reason == DLL_PROCESS_ATTACH {
        setup_dinput8_hook();

        unsafe { AllocConsole() }.unwrap();
        println!("Hello world");
    }
    true.into()
}
