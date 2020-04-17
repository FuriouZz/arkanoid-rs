pub trait EventHandler {
    fn frame(&mut self);
    fn resize(&mut self, _width: i32, _height: i32) {}
    fn pointer_down(&mut self, _x: i32, _height: i32) {}
    fn pointer_up(&mut self, _x: i32, _height: i32) {}
    fn pointer_move(&mut self, _x: i32, _height: i32) {}
}

#[derive(Debug)]
pub enum EventType {
    POINTER_DOWN,
    POINTER_UP,
    POINTER_MOVE,
}

#[derive(Debug)]
pub struct Event {
    pub event: EventType,
    pub values: [i32; 4],
}