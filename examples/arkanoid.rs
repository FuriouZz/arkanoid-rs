mod entities;
// use std::collections::HashSet;

// use fine::{
//     event::{EventHandler, KeyCode},
//     start,
//     math::Rect,
//     wasm::canvas,
// };

// pub struct Stage {
//     state: GameState,
//     player: Player,
//     ball: Ball,
// }

// impl Stage {
//     fn new() -> Self {
//         Self {
//             state: GameState::new(),
//             player: Player::new(),
//             ball: Ball::new(),
//         }
//     }
// }

// pub struct GameState {
//     screen: Rect,
//     pressed: HashSet<KeyCode>,
//     dt: f64,
//     time: f64,
//     sticky: bool,
// }

// impl GameState {
//     fn new() -> Self {
//         let pressed = HashSet::new();

//         let state = Self {
//             screen: Rect::new(),
//             pressed,
//             time: fine::wasm::now(),
//             dt: 0.,
//             sticky: true,
//         };

//         state
//     }
// }

// impl EventHandler for Stage {
//     fn frame(&mut self) {
//         let time = fine::wasm::now();
//         self.state.dt = time - self.state.time;
//         self.state.time = time;

//         if self.state.pressed.contains(&KeyCode::Space) {
//             self.state.sticky = false;
//         }

//         self.player.update(&self.state);
//         self.ball.update(&self.state, &self.player);

//         canvas::clear();
//         self.player.draw();
//         self.ball.draw();

//         self.player.debug();
//         self.ball.debug();
//     }

//     fn resize(&mut self, width: i32, height: i32) {
//         self.state.screen.size.x = width as f64;
//         self.state.screen.size.y = height as f64;
//         self.player.resize(&self.state);
//         self.ball.resize(&self.state);
//         fine::log!("Resolution {}x{}", self.state.screen.size.x, self.state.screen.size.y);
//     }

//     fn focus(&mut self) {
//         self.state.time = fine::wasm::now();
//     }

//     fn key_up(&mut self, keycode: KeyCode) {
//         self.state.pressed.remove(&keycode);
//     }

//     fn key_pressed(&mut self, keycode: KeyCode) {
//         self.state.pressed.insert(keycode);
//     }
// }

pub struct ArkanoidScene {
    triangle: Option<entities::Triangle>,
}

impl fine::Scene for ArkanoidScene {
    fn on_init(
        &mut self,
        window: &fine::Window,
        frame: &wgpu::SwapChainOutput,
        device: &wgpu::Device,
    ) -> Option<wgpu::CommandBuffer> {
        fine::log!("Arkanoid initialized 🥰");
        let triangle = entities::Triangle::new(device);

        let mut encoder =
        device.create_command_encoder(&fine::graphic::wgpu::CommandEncoderDescriptor {
            label: None,
        });

        triangle.draw(&mut encoder, &frame.view);
        self.triangle = Some(triangle);

        Some(encoder.finish())
    }
    fn on_event(&mut self, e: fine::event::Event) {}
    fn on_draw(
        &mut self,
        frame: &wgpu::SwapChainOutput,
        device: &wgpu::Device,
    ) -> Option<wgpu::CommandBuffer> {
        self.triangle.as_ref().map(|triangle| {
            let mut encoder =
                device.create_command_encoder(&fine::graphic::wgpu::CommandEncoderDescriptor {
                    label: None,
                });

            triangle.draw(&mut encoder, &frame.view);
            encoder.finish()
        })
    }
}

fn main() {
    let scene = ArkanoidScene { triangle: None };
    fine::start(scene, Default::default());
}
