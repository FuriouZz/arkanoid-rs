mod wasm;

use std::panic;
use wasm::canvas;

pub fn main() {
  unsafe { wasm::application::Application::init(); }

  {
    panic::set_hook(Box::new(|info| {
      let msg = format!("{:?}", info);
      wasm::console::warn(msg.as_str());
    }));
  }

  canvas::clear();
  canvas::fill_style("red");
  canvas::fill_rect(0, 0, 100, 200);
}