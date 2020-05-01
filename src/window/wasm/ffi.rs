use std::ffi::CString;
use std::os::raw::c_char;
use wasm_bindgen::prelude::*;

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

extern "C" {
  fn performance_now() -> f64;
  fn random() -> f64;
}

pub fn now() -> f64 {
  unsafe { performance_now() }
}

pub fn rand() -> f64 {
  unsafe { random() }
}

pub mod canvas {
  #[repr(C)]
  pub struct Size(pub u32, pub u32);

  extern "C" {
    fn canvas_create();
    fn canvas_get_size() -> Size;
  }

  pub fn create() {
    unsafe { canvas_create() }
  }

  pub fn get_size() -> Size {
    unsafe { canvas_get_size() }
  }
}