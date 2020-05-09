use super::Frame;
use super::event::Event;

pub trait Scene {
    fn on_load(_frame: Frame) -> Self where Self: Sized;
    fn on_start(&mut self, _frame: Frame) {}
    fn on_event(&mut self, _e: Event) {}
    fn on_draw(&mut self, _frame: Frame) {}
}
