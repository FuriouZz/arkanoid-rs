use super::canvas;
use super::console;
use std::sync::Once;
use crate::event::EventHandler;

static mut APP: Option<Application> = None;
static START_APP: Once = Once::new();

pub struct Application {
  stage: Option<Box<dyn EventHandler>>
}

impl Application {

  pub unsafe fn init(handler: Box<dyn EventHandler>) -> &'static mut Self {
    let app = Application::get();
    app.stage = Some(handler);
    app
  }

  unsafe fn get() -> &'static mut Self {
    START_APP.call_once(|| {
      APP = Some(Self {
        stage: None
      });
      console::log("Application initialized 🥰");
    });

    APP.as_mut().expect("Application is not initialized.")
  }

  fn resize(&self, width: i32, height: i32) {
    let app = unsafe { Application::get() };

    match app.stage.as_mut() {
      Some(stage) => stage.resize(width, height),
      None => {}
    };
  }

  fn frame(&self) {
    let app = unsafe { Application::get() };

    match app.stage.as_mut() {
      Some(stage) => stage.frame(),
      None => {}
    };
  }

}

#[no_mangle]
extern "C" fn resize(width: i32, height: i32) {
  unsafe {
    Application::get().resize(width, height);
  }
}

#[no_mangle]
extern "C" fn frame() {
  unsafe {
    Application::get().frame();
  }
}