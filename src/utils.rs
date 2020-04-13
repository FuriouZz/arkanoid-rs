use std::ffi::CString;
use crate::ffi;

pub fn log(s: &str) {
  let msg = CString::new(s).expect("CString::new failed");
  unsafe { ffi::console_log(msg.into_raw(), s.len()); }
}