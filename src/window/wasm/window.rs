use raw_window_handle::{web::WebHandle, HasRawWindowHandle, RawWindowHandle};
use super::ffi;

pub struct Window {
    id: u32,
    pub size: (u32, u32),
}

impl Window {
    pub fn new() -> Self {
        ffi::canvas::create();
        let size = ffi::canvas::get_size();

        Self {
            id: 1,
            size: (size.0, size.1),
        }
    }
}

unsafe impl HasRawWindowHandle for Window {
    fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        let handle = WebHandle {
            id: self.id,
            ..WebHandle::empty()
        };

        RawWindowHandle::Web(handle)
    }
}
