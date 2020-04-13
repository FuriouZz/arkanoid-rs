mod ffi;

pub fn main() {
  ffi::log("Hello World 🤔");
  ffi::clear();
  ffi::fill_style("red");
  ffi::fill_rect(0, 0, 100, 200);
}