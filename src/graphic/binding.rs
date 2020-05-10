pub struct BindingDescriptor(Vec<wgpu::BindGroupLayoutEntry>);

impl BindingDescriptor {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn from_entry(entry: wgpu::BindGroupLayoutEntry) -> Self {
        Self::new().entry(entry)
    }

    pub fn from_entries(entries: &[wgpu::BindGroupLayoutEntry]) -> Self {
        let mut binding = Self::new();
        binding.0.extend_from_slice(entries);
        binding
    }

    /// A description of a single binding inside a bind group.
    pub fn entry(mut self, entry: wgpu::BindGroupLayoutEntry) -> Self {
        self.0.push(entry);
        self
    }

    /// Build bind group layout
    pub fn build(self, device: &wgpu::Device) -> BindingLayout {
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("[fine] bind group layout"),
            bindings: self.0.as_slice(),
        });

        BindingLayout {
            entries: self.0,
            layout,
        }
    }
}

pub struct BindingLayout {
    entries: Vec<wgpu::BindGroupLayoutEntry>,
    layout: wgpu::BindGroupLayout,
}

impl BindingLayout {
    /// Return bind group layout
    pub fn get_layout(&self) -> &wgpu::BindGroupLayout {
        &self.layout
    }

    /// Bind resources to entries
    pub fn bind<'a, F>(&'a self, f: F) -> Vec<wgpu::Binding>
    where
        F: Fn(&wgpu::BindGroupLayoutEntry) -> Option<wgpu::BindingResource<'a>>,
    {
        self.entries
            .iter()
            .filter_map(|entry| match f(entry) {
                Some(resource) => Some(wgpu::Binding {
                    binding: entry.binding,
                    resource,
                }),
                None => None,
            })
            .collect()
    }
}
