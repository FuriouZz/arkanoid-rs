pub mod key;
pub use key::KeyCode;

pub enum Event {
    PointerUp(f32, f32),
    PointerDown(f32, f32),
    PointerMove(f32, f32),
    KeyUp(KeyCode),
    KeyDown(KeyCode),
    KeyPressed(KeyCode),
    Focus,
    Blur,
    Frame,
    RedrawRequested,
    Resize(u32, u32),
}
