use super::canvas;
use super::console;
use std::sync::Once;
use crate::event::{EventHandler, EventType, Event};

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

  fn get() -> &'static mut Self {
    START_APP.call_once(|| {
      unsafe {
        APP = Some(Self {
          stage: None
        });
        console::log("Application initialized 🥰");
      }
    });

    unsafe {
      APP.as_mut().expect("Application is not initialized.")
    }
  }

  fn get_stage(&mut self) -> &mut Box<dyn EventHandler> {
    self.stage.as_mut().expect("No stage found")
  }

  fn resize(&mut self, width: i32, height: i32) {
    let stage = Application::get().get_stage();
    stage.resize(width, height);
  }

  fn frame(&mut self) {
    let stage = Application::get().get_stage();
    stage.frame();
  }

  fn event(&mut self, e: Event) {
    let stage = Application::get().get_stage();

    match e.event {
      EventType::POINTER_DOWN => stage.pointer_down(e.values[0], e.values[1]),
      EventType::POINTER_UP => stage.pointer_up(e.values[0], e.values[1]),
      EventType::POINTER_MOVE => stage.pointer_move(e.values[0], e.values[1]),
      _ => {}
    }
  }

}

#[no_mangle]
extern "C" fn resize(width: i32, height: i32) {
  Application::get().resize(width, height);
}

#[no_mangle]
extern "C" fn frame() {
  Application::get().frame();
}

#[no_mangle]
extern "C" fn pointer(event: EventType, x: i32, y: i32) {
  let e = Event {
    event,
    values: [x,y,0,0],
  };
  Application::get().event(e);
}