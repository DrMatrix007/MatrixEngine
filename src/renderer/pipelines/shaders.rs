use std::borrow::Cow;

use wgpu::ShaderModule;

use super::device_queue::DeviceQueue;

pub struct MatrixShaders {
    module: ShaderModule,
}

impl MatrixShaders {
    pub fn new(device_queue: &DeviceQueue, shader: impl AsRef<str>) -> Self {
        Self {
            module: device_queue
                .device()
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("matrix shaders"),
                    source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(shader.as_ref())),
                }),
        }
    }

    pub(crate) fn module(&self) -> &ShaderModule {
        &self.module
    }
}
