mod application;
pub mod canvas;
pub mod console;
pub mod key;
pub use application::Application;

extern "C" {
  fn performance_now() -> f64;
  fn random() -> f64;
}

pub fn now() -> f64 {
  unsafe { performance_now() }
}

pub fn rand() -> f64 {
  unsafe { random() }
}