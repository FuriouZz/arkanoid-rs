pub trait Scene {
    fn on_init(&mut self, frame: crate::frame::Frame);
    fn on_event(&mut self, e: crate::event::Event);
    fn on_draw(&mut self, frame: crate::frame::Frame);
}
