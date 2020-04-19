use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
extern "C" {
    fn console_log(ptr: *mut c_char, len: usize);
    fn console_warn(ptr: *mut c_char, len: usize);
}

pub fn log(s: &str) {
    let cs = CString::new(s).expect("CString::new failed");
    unsafe { console_log(cs.into_raw(), s.len()) }
}

pub fn warn(s: &str) {
    let cs = CString::new(s).expect("CString::new failed");
    unsafe { console_warn(cs.into_raw(), s.len()) }
}

#[macro_export]
macro_rules! log {
  ($($t:tt)*) => ($crate::wasm::console::log(format!($($t)*).as_str()))
}

#[macro_export]
macro_rules! warn {
  ($($t:tt)*) => ($crate::wasm::console::warn(format!($($t)*).as_str()))
}
