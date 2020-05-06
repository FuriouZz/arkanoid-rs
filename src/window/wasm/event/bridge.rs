use super::super::ffi;
use crate::event::Event;
use std::sync::Once;
use wasm_bindgen::prelude::*;

static mut BRIDGE: Option<Bridge> = None;
static START_BRIDGE: Once = Once::new();

pub struct Bridge {
    handler: Option<crate::context::Context>,
}

impl Bridge {
    pub(crate) unsafe fn init<F>(init: F) -> &'static mut Self
    where
        F: FnOnce() -> crate::context::Context,
    {
        let bridge = Bridge::get();
        let handler = init();
        bridge.handler = Some(handler);
        ffi::log("Bridge initialized 🔧");
        bridge.get_handler().init();
        bridge
    }

    pub fn get() -> &'static mut Self {
        START_BRIDGE.call_once(|| unsafe {
            BRIDGE = Some(Self { handler: None });
        });

        unsafe { BRIDGE.as_mut().expect("Application is not initialized.") }
    }

    fn get_handler(&mut self) -> &mut crate::context::Context {
        self.handler.as_mut().expect("No stage found")
    }

    fn on_event(&mut self, e: Event) {
        let handler = Bridge::get().get_handler();
        handler.on_event(e);
    }
}

#[wasm_bindgen]
pub fn on_resize(width: u32, height: u32) {
    let bridge = Bridge::get();
    bridge.on_event(Event::Resize(width, height));
}

#[wasm_bindgen]
pub fn on_frame() {
    let bridge = Bridge::get();
    bridge.on_event(Event::Frame);
}

#[wasm_bindgen]
pub fn on_focus() {
    let bridge = Bridge::get();
    bridge.on_event(Event::Focus);
}

#[wasm_bindgen]
pub fn on_blur() {
    let bridge = Bridge::get();
    bridge.on_event(Event::Blur);
}

#[wasm_bindgen]
pub fn on_pointer_move(x: i32, y: i32) {
    let bridge = Bridge::get();
    bridge.on_event(Event::PointerMove(x as f32, y as f32));
}

#[wasm_bindgen]
pub fn on_pointer_up(x: i32, y: i32) {
    let bridge = Bridge::get();
    bridge.on_event(Event::PointerUp(x as f32, y as f32));
}

#[wasm_bindgen]
pub fn on_pointer_down(x: i32, y: i32) {
    let bridge = Bridge::get();
    bridge.on_event(Event::PointerDown(x as f32, y as f32));
}

#[wasm_bindgen]
pub fn on_key_pressed(key: super::key::KeyCode) {
    let bridge = Bridge::get();
    bridge.on_event(Event::KeyPressed(key.into()));
}

#[wasm_bindgen]
pub fn on_key_up(key: super::key::KeyCode) {
    let bridge = Bridge::get();
    bridge.on_event(Event::KeyUp(key.into()));
}

#[wasm_bindgen]
pub fn on_key_down(key: super::key::KeyCode) {
    let bridge = Bridge::get();
    bridge.on_event(Event::KeyDown(key.into()));
}
