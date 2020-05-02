use super::ffi;
use raw_window_handle::{web::WebHandle, HasRawWindowHandle, RawWindowHandle};

pub type Window = ffi::Canvas;

unsafe impl HasRawWindowHandle for Window {
    fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        let handle = WebHandle {
            id: self.id(),
            ..WebHandle::empty()
        };

        RawWindowHandle::Web(handle)
    }
}
