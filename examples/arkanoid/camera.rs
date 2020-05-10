use fine::math::{Matrix4, Orthographic3, Perspective3};

#[derive(Debug)]
pub struct Camera {
    pub lens: Lens,
    view: Matrix4<f32>,
}

impl Camera {
    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        let o = Orthographic3::new(left, right, bottom, top, near, far);
        Self {
            lens: Lens::Orthographic(o),
            view: Matrix4::identity(),
        }
    }

    pub fn perspective(aspect: f32, fovy: f32, near: f32, far: f32) -> Self {
        let o = Perspective3::new(aspect, fovy, near, far);
        Self {
            lens: Lens::Perspective(o),
            view: Matrix4::identity(),
        }
    }

    pub fn get_view(&self) -> &Matrix4<f32> {
        &self.view
    }

    pub fn get_projection(&self) -> Matrix4<f32> {
        self.lens.get_projection()
    }

    pub fn model_view(&self, model: &Matrix4<f32>) -> Matrix4<f32> {
        self.view * model
    }

    pub fn model_view_projection(&self, model: &Matrix4<f32>) -> Matrix4<f32> {
        self.lens.get_projection() * self.view * model
    }
}

#[derive(Debug)]
pub enum Lens {
    Orthographic(Orthographic3<f32>),
    Perspective(Perspective3<f32>),
}

impl Lens {
    // Return projection matrix
    // WARNING: Due to the new Vulkan coordinate system, we need correction.
    // Read: https://matthewwellings.com/blog/the-new-vulkan-coordinate-system/
    pub fn get_projection(&self) -> Matrix4<f32> {
        match self {
            Self::Orthographic(o) => {
                Matrix4::from_row_slice(&fine::graphic::OPENGL_TO_WGPU_MATRIX) * o.as_matrix()
            }
            Self::Perspective(p) => {
                Matrix4::from_row_slice(&fine::graphic::OPENGL_TO_WGPU_MATRIX) * p.as_matrix()
            }
        }
    }
}
