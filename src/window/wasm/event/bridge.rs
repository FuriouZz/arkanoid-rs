use super::super::ffi;
use crate::event::{EventHandler, Event, KeyCode};
use std::sync::Once;

static mut BRIDGE: Option<Bridge> = None;
static START_BRIDGE: Once = Once::new();

pub struct Bridge {
    handler: Option<Box<dyn EventHandler>>,
}

impl Bridge {
    pub unsafe fn init<T, F>(init: F) -> &'static mut Self
    where
        T: EventHandler + 'static,
        F: FnOnce() -> T,
    {
        let bridge = Bridge::get();
        let handler = Box::new(init());
        bridge.handler = Some(handler);
        bridge
    }

    pub fn get() -> &'static mut Self {
        START_BRIDGE.call_once(|| unsafe {
            BRIDGE = Some(Self { handler: None });
            ffi::log("Application initialized 🥰");
        });

        unsafe { BRIDGE.as_mut().expect("Application is not initialized.") }
    }

    fn get_handler(&mut self) -> &mut Box<dyn EventHandler> {
        self.handler.as_mut().expect("No stage found")
    }

    fn on_event(&mut self, e: Event) {
        let handler = Bridge::get().get_handler();
        handler.on_event(e);
    }
}

#[no_mangle]
extern "C" fn resize(width: u32, height: u32) {
    let bridge = Bridge::get();
    bridge.on_event(Event::Resize(width, height));
}

#[no_mangle]
extern "C" fn frame() {
    let bridge = Bridge::get();
    bridge.on_event(Event::Frame);
}

#[no_mangle]
extern "C" fn focus() {
    let bridge = Bridge::get();
    bridge.on_event(Event::Focus);
}

#[no_mangle]
extern "C" fn blur() {
    let bridge = Bridge::get();
    bridge.on_event(Event::Blur);
}

#[no_mangle]
extern "C" fn pointer_move(x: i32, y: i32) {
    let bridge = Bridge::get();
    bridge.on_event(Event::PointerMove(x as f32, y as f32));
}

#[no_mangle]
extern "C" fn pointer_up(x: i32, y: i32) {
    let bridge = Bridge::get();
    bridge.on_event(Event::PointerUp(x as f32, y as f32));
}

#[no_mangle]
extern "C" fn pointer_down(x: i32, y: i32) {
    let bridge = Bridge::get();
    bridge.on_event(Event::PointerDown(x as f32, y as f32));
}

#[no_mangle]
extern "C" fn key_pressed(key: KeyCode) {
    let bridge = Bridge::get();
    bridge.on_event(Event::KeyPressed(key));
}

#[no_mangle]
extern "C" fn key_up(key: KeyCode) {
    let bridge = Bridge::get();
    bridge.on_event(Event::KeyUp(key));
}

#[no_mangle]
extern "C" fn key_down(key: KeyCode) {
    let bridge = Bridge::get();
    bridge.on_event(Event::KeyDown(key));
}