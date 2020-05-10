use super::event::Event;
use super::Frame;

pub trait Scene {
    fn on_load(_frame: Frame) -> Self
    where
        Self: Sized;
    fn on_start(&mut self, _frame: &mut Frame) {}
    fn on_event(&mut self, _e: Event) {}
    fn on_draw(&mut self, _frame: &mut Frame) {}
}
