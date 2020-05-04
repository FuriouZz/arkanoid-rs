pub struct Binding {
    entries: Vec<wgpu::BindGroupLayoutEntry>,
    layout: Option<wgpu::BindGroupLayout>,
}

impl Binding {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            layout: None,
        }
    }

    /// A description of a single binding inside a bind group.
    pub fn entry(&mut self, entry: wgpu::BindGroupLayoutEntry) -> &mut Self {
        self.entries.push(entry);
        self
    }

    pub fn get_layout(&self) -> &wgpu::BindGroupLayout {
        self.layout.as_ref().expect("[Binding] Layout not built yet.")
    }

    pub fn build(&mut self, device: &wgpu::Device, label: Option<&str>) -> &mut Self {
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label,
            bindings: self.entries.as_slice(),
        });
        self.layout = Some(layout);
        self
    }

    pub fn get_entries(&self) -> std::slice::Iter<wgpu::BindGroupLayoutEntry> {
        self.entries.iter()
    }
}
