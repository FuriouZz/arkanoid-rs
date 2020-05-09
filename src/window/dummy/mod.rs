pub struct Window {}

impl Window {
    pub fn new() -> Self {
        Self {}
    }
    pub fn ready(&self) {}
    pub fn resize(&self, _width: u32, _height: u32) {}
    pub fn width(&self) -> u32 {
        0
    }
    pub fn height(&self) -> u32 {
        0
    }
}

#[cfg(target_os = "macos")]
unsafe impl raw_window_handle::HasRawWindowHandle for Window {
    fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        let handle = raw_window_handle::macos::MacOSHandle {
            ..raw_window_handle::macos::MacOSHandle::empty()
        };

        raw_window_handle::RawWindowHandle::MacOS(handle)
    }
}

#[macro_export]
macro_rules! log {
    ($($t:tt)*) => (println!($($t)*))
}

pub fn now() -> f64 {
    0.
}

pub fn rand() -> f64 {
    0.
}
