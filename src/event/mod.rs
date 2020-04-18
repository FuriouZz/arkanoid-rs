pub mod key;
pub use key::KeyCode;

pub trait EventHandler {
    fn init(&mut self);
    fn frame(&mut self);
    fn resize(&mut self, _width: i32, _height: i32) {}
    fn pointer_up(&mut self, _x: i32, _y: i32) {}
    fn pointer_down(&mut self, _x: i32, _y: i32) {}
    fn pointer_move(&mut self, _x: i32, _y: i32) {}
    fn key_up(&mut self, _keycode: KeyCode) {}
    fn key_down(&mut self, _keycode: KeyCode) {}
    fn key_pressed(&mut self, _keycode: KeyCode) {}
}

#[derive(Debug)]
pub enum EventType {
    PointerUp,
    PointerDown,
    PointerMove,
    KeyUp,
    KeyDown,
    KeyPressed,
}

#[derive(Debug)]
pub struct Event {
    pub event: EventType,
    pub values: [i32; 4],
    pub keycode: crate::wasm::key::KeyCode,
}