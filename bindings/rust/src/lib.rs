use std::ffi;

unsafe extern "C" {
    fn Install(version: *const ffi::c_char);
    fn Create(name: *const ffi::c_char, eula: bool);
    fn Start(name: *const ffi::c_char, memory: *const ffi::c_char);
}

pub fn install(version: &str) {
    let version = ffi::CString::new(version).expect("");
    unsafe { Install(version.as_ptr()) }
}

pub fn create(name: &str, eula: bool) {
    let name = ffi::CString::new(name).expect("");
    unsafe { Create(name.as_ptr(), eula) }
}

pub fn start(name: &str, memory: &str) {
    let name = ffi::CString::new(name).expect("");
    let memory = ffi::CString::new(memory).expect("");
    unsafe { Start(name.as_ptr(), memory.as_ptr()) }
}
