use fine::{
    event, start,
    wasm::{canvas, console},
};

struct Stage {
    width: i32,
    height: i32,
    x: i32
}

impl event::EventHandler for Stage {
    fn frame(&mut self) {
        canvas::clear();
        canvas::fill_style("red");
        canvas::fill_rect((self.x - 50) as usize, (self.height - 20) as usize, 100, 20);
    }

    fn resize(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
        console::log(format!("Resolution {}x{} from Stage", width, height).as_str())
    }

    fn pointer_move(&mut self, x: i32, y: i32) {
        self.x = x;
    }
}

pub fn main() {
    start(Stage {
        width: 0,
        height: 0,
        x: 0,
    });
}
