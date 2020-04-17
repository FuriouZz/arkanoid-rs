use fine::{
    event, start,
    wasm::{canvas, console},
};

struct Stage;

impl event::EventHandler for Stage {
    fn frame(&mut self) {
        canvas::clear();
        canvas::fill_style("red");
        canvas::fill_rect(0, 0, 100, 200);
    }

    fn resize(&mut self, width: i32, height: i32) {
        console::log(format!("Resolution {}x{} from Stage", width, height).as_str())
    }
}

pub fn main() {
    start(Stage);
}
