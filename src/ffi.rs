use std::os::raw::c_char;
use std::ffi::CString;

#[no_mangle]
extern "C" {
  pub fn console_log(ptr: *mut c_char, len: usize);

  // Canvas
  pub fn canvas_clear();
  pub fn canvas_fill_rect(x: usize, y: usize, width: usize, height: usize);
  pub fn canvas_fill_style(ptr: *mut c_char, len: usize);
}

pub fn log(s: &str) {
  let cs = CString::new(s).expect("CString::new failed");
  unsafe { console_log(cs.into_raw(), s.len()); }
}

pub fn clear() {
  unsafe { canvas_clear() }
}

pub fn fill_rect(x: usize, y: usize, width: usize, height: usize) {
  unsafe { canvas_fill_rect(x, y, width, height) }
}

pub fn fill_style(s: &str) {
  let cs = CString::new(s).expect("CString::new failed");
  unsafe { canvas_fill_style(cs.into_raw(), s.len()) }
}