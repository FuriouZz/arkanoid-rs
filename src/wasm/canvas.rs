use std::os::raw::c_char;
use std::ffi::CString;

#[no_mangle]
extern "C" {
  fn canvas_clear();
  fn canvas_fill_rect(x: usize, y: usize, width: usize, height: usize);
  fn canvas_fill_style(ptr: *mut c_char, len: usize);
  fn canvas_stroke_rect(x: usize, y: usize, width: usize, height: usize);
  fn canvas_stroke_style(ptr: *mut c_char, len: usize);
  fn canvas_translate(x: usize, y: usize);
  fn canvas_scale(x: usize, y: usize);
  fn canvas_rotate(angle: usize);
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

pub fn stroke_rect(x: usize, y: usize, width: usize, height: usize) {
  unsafe { canvas_stroke_rect(x, y, width, height) }
}

pub fn stroke_style(s: &str) {
  let cs = CString::new(s).expect("CString::new failed");
  unsafe { canvas_stroke_style(cs.into_raw(), s.len()) }
}

pub fn translate(x: usize, y: usize) {
  unsafe { canvas_translate(x, y) }
}

pub fn scale(x: usize, y: usize) {
  unsafe { canvas_scale(x, y) }
}

pub fn rotate(angle: usize) {
  unsafe { canvas_rotate(angle) }
}