use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    pub fn warn(s: &str);
}

#[wasm_bindgen(module = "/public/imports.js")]
extern "C" {
    pub type Canvas;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Canvas;

    #[wasm_bindgen(method, getter)]
    pub fn id(this: &Canvas) -> u32;

    #[wasm_bindgen(method, getter)]
    pub fn width(this: &Canvas) -> u32;

    #[wasm_bindgen(method, getter)]
    pub fn height(this: &Canvas) -> u32;

    #[wasm_bindgen(method)]
    pub fn resize(this: &Canvas, width: u32, height: u32);

    pub fn now() -> f64;
    pub fn rand() -> f64;
}

#[macro_export]
macro_rules! log {
  ($($t:tt)*) => ($crate::ffi::log(format!($($t)*).as_str()))
}

#[macro_export]
macro_rules! warn {
  ($($t:tt)*) => ($crate::ffi::warn(format!($($t)*).as_str()))
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    // alert(&format!("Hello, {}!", name));
}
