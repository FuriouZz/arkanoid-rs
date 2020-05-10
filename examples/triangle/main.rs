use fine::graphic::wgpu;

pub struct TriangleExample {
    pipeline: wgpu::RenderPipeline,
}

impl fine::Scene for TriangleExample {
    fn on_load(mut frame: fine::Frame) -> Self
    where
        Self: Sized,
    {
        fine::log!("🚧 TriangleExample is loading");

        let gpu = frame.gpu();

        // Create shader modules
        let vertex_module = gpu.create_shader_module(&include_bytes!("./triangle.vert.spv")[..]);
        let fragment_module = gpu.create_shader_module(&include_bytes!("./triangle.frag.spv")[..]);

        // Setup pipeline layout
        let pipeline_layout = gpu
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[],
            });

        // Create pipeline
        let pipeline = gpu
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                // Set layout
                layout: &pipeline_layout,

                // Set vertex module
                vertex_stage: wgpu::ProgrammableStageDescriptor {
                    module: &vertex_module,
                    entry_point: "main",
                },

                // Set fragment module
                fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                    module: &fragment_module,
                    entry_point: "main",
                }),

                // Define a rasterization state
                rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: wgpu::CullMode::Front,
                    depth_bias: 0,
                    depth_bias_clamp: 0.,
                    depth_bias_slope_scale: 0.,
                }),

                // Draw triangles
                primitive_topology: wgpu::PrimitiveTopology::TriangleList,

                // Color states (blending, write, format...)
                color_states: &[wgpu::ColorStateDescriptor {
                    format: fine::graphic::DEFAULT_TEXTURE_FORMAT,
                    color_blend: wgpu::BlendDescriptor::REPLACE,
                    alpha_blend: wgpu::BlendDescriptor::REPLACE,
                    write_mask: wgpu::ColorWrite::ALL,
                }],

                // Depth stencil (example soon!)
                depth_stencil_state: None,

                // Well no vertex this time!
                vertex_state: wgpu::VertexStateDescriptor {
                    index_format: wgpu::IndexFormat::Uint16,
                    vertex_buffers: &[],
                },

                // Draw one instance
                sample_count: 1,
                sample_mask: !0,
                alpha_to_coverage_enabled: false,
            });

        Self { pipeline }
    }

    fn on_start(&mut self, _frame: &mut fine::Frame) {
        fine::log!("TriangleExample initialized 🥰");
    }

    fn on_draw(&mut self, frame: &mut fine::Frame) {
        let (gpu, view) = frame.target();
        let encoder = &mut gpu.encoder;

        // Prepare render pass
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: view,
                resolve_target: None,
                load_op: wgpu::LoadOp::Clear,
                store_op: wgpu::StoreOp::Store,
                clear_color: wgpu::Color {
                    r: 0.,
                    g: 0.,
                    b: 0.,
                    a: 1.,
                },
            }],
            depth_stencil_attachment: None,
        });

        // Set pipeline
        pass.set_pipeline(&self.pipeline);

        // Draw!
        pass.draw(0..3, 0..1);

        // >> This is hidden by my framework, but here, the encoder is submitted to the render queue!
    }
}

fn main() {
    fine::start::<TriangleExample>(Default::default());
}
