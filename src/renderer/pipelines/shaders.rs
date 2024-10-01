use std::{borrow::Cow, marker::PhantomData};

use wgpu::ShaderModule;

use super::{device_queue::DeviceQueue, vertecies::Vertexable};

pub struct MatrixShaders<Vertex: Vertexable> {
    module: ShaderModule,
    marker: PhantomData<Vertex>,
}

impl<Vertex: Vertexable> MatrixShaders<Vertex> {
    pub fn new(device_queue: &DeviceQueue, shader: impl AsRef<str>) -> Self {
        Self {
            module: device_queue
                .device()
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("matrix shaders"),
                    source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(shader.as_ref())),
                }),
            marker: PhantomData,
        }
    }

    pub(crate) fn module(&self) -> &ShaderModule {
        &self.module
    }
}
