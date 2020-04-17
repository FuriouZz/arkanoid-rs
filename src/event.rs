pub trait EventHandler {
    fn frame(&mut self);
    fn resize(&mut self, _width: i32, _height: i32) {}
}