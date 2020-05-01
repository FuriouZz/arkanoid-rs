pub struct Binding {
    binding: u32,
    entries: Vec<wgpu::BindGroupLayoutEntry>,
}

impl Binding {
    pub fn new() -> Self {
        Self {
            binding: 0,
            entries: Vec::new(),
        }
    }

    /// A description of a single binding inside a bind group.
    pub fn entry(&mut self, visibility: wgpu::ShaderStage, ty: wgpu::BindingType) {
        self.entries.push(wgpu::BindGroupLayoutEntry {
            binding: self.binding,
            visibility,
            ty,
        });
        self.binding += 1;
    }

    /// A buffer for uniform values.
    ///
    /// Example GLSL syntax:
    /// ```
    /// layout(std140, binding = 0)
    /// uniform Globals {
    ///     vec2 aUniform;
    ///     vec2 anotherUniform;
    /// };
    /// ```
    pub fn uniform_buffer(&mut self, visibility: wgpu::ShaderStage, dynamic: bool) {
        self.entry(visibility, wgpu::BindingType::UniformBuffer { dynamic });
    }

    /// A storage buffer.
    ///
    /// Example GLSL syntax:
    /// ```
    /// layout (set=0, binding=0) buffer myStorageBuffer {
    ///     vec4 myElement[];
    /// };
    /// ```
    pub fn storage_buffer(&mut self, visibility: wgpu::ShaderStage, dynamic: bool, readonly: bool) {
        self.entry(
            visibility,
            wgpu::BindingType::StorageBuffer { dynamic, readonly },
        );
    }

    /// A sampler that can be used to sample a texture.
    ///
    /// Example GLSL syntax:
    /// ```
    /// layout(binding = 0)
    /// uniform sampler s;
    /// ```
    pub fn sampler(&mut self, visibility: wgpu::ShaderStage, comparison: bool) {
        self.entry(visibility, wgpu::BindingType::Sampler { comparison });
    }

    /// A storage texture.
    /// Example GLSL syntax:
    /// ```
    /// layout(set=0, binding=0, r32f) uniform image2D myStorageImage;
    /// ```
    pub fn storage_texture(
        &mut self,
        visibility: wgpu::ShaderStage,
        format: wgpu::TextureFormat,
        component_type: wgpu::TextureComponentType,
        dimension: wgpu::TextureViewDimension,
        readonly: bool,
    ) {
        self.entry(
            visibility,
            wgpu::BindingType::StorageTexture {
                readonly,
                dimension,
                format,
                component_type,
            },
        );
    }

    /// A texture.
    ///
    /// Example GLSL syntax:
    /// ```
    /// layout(binding = 0)
    /// uniform texture2D t;
    /// ```
    pub fn sampled_texture(
        &mut self,
        visibility: wgpu::ShaderStage,
        component_type: wgpu::TextureComponentType,
        dimension: wgpu::TextureViewDimension,
        multisampled: bool,
    ) {
        self.entry(
            visibility,
            wgpu::BindingType::SampledTexture {
                component_type,
                dimension,
                multisampled,
            },
        );
    }

    pub fn get_layout<'a>(&'a self, label: Option<&'a str>) -> wgpu::BindGroupLayoutDescriptor<'a> {
        wgpu::BindGroupLayoutDescriptor {
            label,
            bindings: self.entries.as_slice(),
        }
    }

    pub fn build_layout(&self, device: &wgpu::Device) -> wgpu::BindGroupLayout {
        let layout = self.get_layout(None);
        device.create_bind_group_layout(&layout)
    }

    pub fn bind<F>(&self, f: F) -> Vec<wgpu::Binding>
    where
        F: Fn(&wgpu::BindGroupLayoutEntry) -> Option<wgpu::BindingResource>,
    {
        self.entries
            .iter()
            .filter_map(|entry| {
                let r = f(entry);
                match r {
                    Some(resource) => Some(wgpu::Binding {
                        binding: entry.binding,
                        resource,
                    }),
                    None => None,
                }
            })
            .collect()
    }
}