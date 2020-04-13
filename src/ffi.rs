use std::os::raw::c_char;

#[no_mangle]
extern "C" {
  pub fn console_log(ptr: *mut c_char, len: usize);
}