use std::os::raw::c_char;
use std::ffi::CString;

#[no_mangle]
extern "C" {
    fn canvas_clear();
    fn canvas_fill_rect(x: f64, y: f64, width: f64, height: f64);
    fn canvas_fill_style(ptr: *mut c_char, len: usize);
    fn canvas_stroke_rect(x: f64, y: f64, width: f64, height: f64);
    fn canvas_stroke_style(ptr: *mut c_char, len: usize);
    fn canvas_translate(x: f64, y: f64);
    fn canvas_scale(x: f64, y: f64);
    fn canvas_rotate(angle: f64);
    fn canvas_restore();
    fn canvas_save();
    fn canvas_set_transform(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64);
    fn canvas_reset_transform();
    fn canvas_stroke();
    fn canvas_fill();
    fn canvas_fill_outside();
    fn canvas_ellipse(x: f64, y: f64, radiusX: f64, radiusY: f64, rotation: f64, startAngle: f64, endAngle: f64, anticlockwise: bool);
    fn canvas_circle(x: f64, y: f64, radius: f64);
    fn canvas_global_alpha(alpha: f64);
    fn canvas_move_to(x: f64, y: f64);
    fn canvas_line_to(x: f64, y: f64);
    fn canvas_begin_path();
    fn canvas_close_path();
    fn canvas_arc(x: f64, y: f64, radius: f64, startAngle: f64, endAngle: f64, anticlockwise: bool);
    fn canvas_quadratic_curve_to(cpx: f64, cpy: f64, x: f64, y: f64);
    fn canvas_bezier_curve_to(cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64);
    fn canvas_is_point_inside_path(x: f64, y: f64);
    fn canvas_is_point_outside_path(x: f64, y: f64);
    fn canvas_create_gradient() -> usize;
    fn canvas_linear_gradient(index: usize, x0: f64, y0: f64, x1: f64, y1: f64);
    fn canvas_radial_gradient(index: usize, x0: f64, y0: f64, r0: f64, x1: f64, y1: f64, r1: f64);
    fn canvas_fill_style_gradient(index: usize);
    fn canvas_stroke_style_gradient(index: usize);
    fn canvas_add_color_stop(index: usize, offset: f64, ptr: *mut c_char, len: usize);
}

pub fn clear() {
  unsafe { canvas_clear() }
}

pub fn fill_rect(x: f64, y: f64, width: f64, height: f64) {
  unsafe { canvas_fill_rect(x, y, width, height) }
}

pub fn fill_style(s: &str) {
  let cs = CString::new(s).expect("CString::new failed");
  unsafe { canvas_fill_style(cs.into_raw(), s.len()) }
}

pub fn stroke_rect(x: f64, y: f64, width: f64, height: f64) {
  unsafe { canvas_stroke_rect(x, y, width, height) }
}

pub fn stroke_style(s: &str) {
  let cs = CString::new(s).expect("CString::new failed");
  unsafe { canvas_stroke_style(cs.into_raw(), s.len()) }
}

pub fn translate(x: f64, y: f64) {
  unsafe { canvas_translate(x, y) }
}

pub fn scale(x: f64, y: f64) {
  unsafe { canvas_scale(x, y) }
}

pub fn rotate(angle: f64) {
  unsafe { canvas_rotate(angle) }
}

pub fn restore() {
    unsafe { canvas_restore() }
}

pub fn save() {
    unsafe { canvas_save() }
}

pub fn set_transform(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
    unsafe { canvas_set_transform(a,b,c,d,e,f) }
}

pub fn reset_transform() {
    unsafe { canvas_reset_transform() }
}

pub fn stroke() {
    unsafe { canvas_stroke() }
}

pub fn fill() {
    unsafe { canvas_fill() }
}

pub fn fill_outside() {
    unsafe { canvas_fill_outside() }
}

pub fn ellipse(x: f64, y: f64, radius_x: f64, radius_y: f64, rotation: f64, start_angle: f64, end_angle: f64, anticlockwise: bool) {
    unsafe { canvas_ellipse(x, y, radius_x, radius_y, rotation, start_angle, end_angle, anticlockwise) }
}

pub fn circle(x: f64, y: f64, radius: f64) {
    unsafe { canvas_circle(x, y, radius) }
}

pub fn global_alpha(alpha: f64) {
    unsafe { canvas_global_alpha(alpha) }
}

pub fn move_to(x: f64, y: f64) {
    unsafe { canvas_move_to(x, y) }
}

pub fn line_to(x: f64, y: f64) {
    unsafe { canvas_line_to(x, y) }
}

pub fn begin_path() {
    unsafe { canvas_begin_path() }
}

pub fn close_path() {
    unsafe { canvas_close_path() }
}

pub fn arc(x: f64, y: f64, radius: f64, start_angle: f64, end_angle: f64, anticlockwise: bool) {
    unsafe { canvas_arc(x, y, radius, start_angle, end_angle, anticlockwise) }
}

pub fn quadratic_curve_to(cpx: f64, cpy: f64, x: f64, y: f64) {
    unsafe { canvas_quadratic_curve_to(cpx, cpy, x, y) }
}

pub fn bezier_curve_to(cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) {
    unsafe { canvas_bezier_curve_to(cp1x, cp1y, cp2x, cp2y, x, y) }
}

pub fn is_point_inside_path(x: f64, y: f64) {
    unsafe { canvas_is_point_inside_path(x, y) }
}

pub fn is_point_outside_path(x: f64, y: f64) {
    unsafe { canvas_is_point_outside_path(x, y) }
}


pub struct Gradient(usize);

impl Gradient {
    pub fn new() -> Self {
        let index = unsafe { canvas_create_gradient() };
        Self(index)
    }

    pub fn linear(&self, x0: f64, y0: f64, x1: f64, y1: f64) {
        unsafe { canvas_linear_gradient(self.0, x0, y0, x1, y1) }
    }

    pub fn radial(&self, x0: f64, y0: f64, r0: f64, x1: f64, y1: f64, r1: f64) {
        unsafe { canvas_radial_gradient(self.0, x0, y0, r0, x1, y1, r1) }
    }

    pub fn add_color_stop(&self, offset: f64, color: &str) {
        let cs = CString::new(color).expect("CString::new failed");
        unsafe { canvas_add_color_stop(self.0, offset, cs.into_raw(), color.len()) }
    }

    pub fn fill(&self) {
        unsafe { canvas_fill_style_gradient(self.0) }
    }

    pub fn stroke(&self) {
        unsafe { canvas_stroke_style_gradient(self.0) }
    }
}