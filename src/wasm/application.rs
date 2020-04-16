use super::canvas;
use super::console;
use std::sync::Once;

static mut APP: Option<Application> = None;
static START_APP: Once = Once::new();

pub struct Application {}

impl Application {

  pub unsafe fn init() -> &'static mut Self {
    Application::get()
  }

  unsafe fn get() -> &'static mut Self {
    START_APP.call_once(|| {
      let app = Self {};
      APP = Some(app);
      console::log("Application initialized 🥰");
    });

    APP.as_mut().expect("Application is not initialized.")
  }

  fn resize(&self, width: i32, height: i32) {
    console::log(format!("Resolution {}x{}", width, height).as_str());
  }

  fn update(&self) {
    // console::log("update");
  }

}

#[no_mangle]
pub extern "C" fn resize(width: i32, height: i32) {
  unsafe {
    Application::get().resize(width, height);
  }
}

#[no_mangle]
pub extern "C" fn update() {
  unsafe {
    Application::get().update();
  }
}