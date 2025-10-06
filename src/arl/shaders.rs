use crate::arl::device_queue::DeviceQueue;

pub struct ShadersArgs {
    pub shaders: String,
    pub vertex_entry: String,
    pub fragment_entry: Option<String>,
}

pub struct Shaders {
    shaders: wgpu::ShaderModule,
    vertex_entry: String,
    fragment_entry: Option<String>,
}

impl Shaders {
    pub fn new(label: &str, args: ShadersArgs, device_queue: &DeviceQueue) -> Self {
        Self {
            shaders: device_queue
                .device()
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some(label),
                    source: wgpu::ShaderSource::Wgsl(args.shaders.into()),
                }),
            fragment_entry: args.fragment_entry,
            vertex_entry: args.vertex_entry,
        }
    }
    pub fn raw(&self) -> &wgpu::ShaderModule {
        &self.shaders
    }

    pub fn vertex_entry(&self) -> &str {
        &self.vertex_entry
    }

    pub fn fragment_entry(&self) -> Option<&str> {
        self.fragment_entry.as_deref()
    }
}
