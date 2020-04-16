use std::os::raw::c_char;

#[no_mangle]
extern "C" {
  fn console_log(ptr: *mut c_char, len: usize);
  fn console_warn(ptr: *mut c_char, len: usize);
}

pub mod console {
  use std::ffi::CString;

  pub fn log(s: &str) {
    let cs = CString::new(s).expect("CString::new failed");
    unsafe { super::console_log(cs.into_raw(), s.len()) }
  }

  pub fn warn(s: &str) {
    let cs = CString::new(s).expect("CString::new failed");
    unsafe { super::console_warn(cs.into_raw(), s.len()) }
  }
}