use fine::{
    event, start,
    wasm::{canvas, console},
    wasm::canvas::Gradient,
};

struct Stage {
    width: f64,
    height: f64,
    x: f64,
    gradient: Option<Gradient>
}

impl Stage {

    fn create_gradient(&mut self) {
        if let Some(g) = self.gradient.as_ref() {
            g.linear(0., 0., self.width, 0.);
            g.add_color_stop(0., "red");
            g.add_color_stop(0.5, "white");
            g.add_color_stop(1., "red");
        } else {
            self.gradient = Some(Gradient::new());
            self.create_gradient();
        }
    }

}

impl event::EventHandler for Stage {

    fn init(&mut self) {
        self.create_gradient()
    }

    fn frame(&mut self) {
        canvas::clear();
        self.gradient.as_ref().unwrap().fill();
        canvas::fill_rect(self.x - 50., self.height - 20., 100., 20.);
    }

    fn resize(&mut self, width: i32, height: i32) {
        self.width = width as f64;
        self.height = height as f64;
        console::log(format!("Resolution {}x{} from Stage", width, height).as_str());
        self.create_gradient();
    }

    fn pointer_move(&mut self, x: i32, _y: i32) {
        self.x = x as f64;
    }
}

pub fn main() {
    start(Stage {
        width: 0.,
        height: 0.,
        x: 0.,
        gradient: None
    });
}
