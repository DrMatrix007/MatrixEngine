use std::{
    fs::{self},
    sync::Arc,
};

use wgpu::{Device, ShaderModuleDescriptor};

#[derive(Clone)]
pub struct MatrixShaders {
    module: Arc<wgpu::ShaderModule>,
}

impl MatrixShaders {
    pub fn module(&self) -> &wgpu::ShaderModule {
        &self.module
    }
}

pub struct ShaderConfig {
    pub vertex_main: String,
    pub fragment_main: String,
}

impl ShaderConfig {
    pub fn vertex_entry(&self) -> &str {
        &self.vertex_main
    }
    pub fn fragment_entry(&self) -> &str {
        &self.fragment_main
    }
}

#[derive(Default, Clone, Copy, Debug)]
pub struct FileNotFound;

impl MatrixShaders {
    pub fn new(device: &Device, filename: String, label: &str) -> Result<Self, FileNotFound> {
        let Ok(shader) = fs::read_to_string(filename) else {
            return Err(FileNotFound);
        };
        Ok(Self::from_string(device, &shader, label))
    }
    pub fn from_string(device: &Device, shader: &str, label: &str) -> Self {
        let module = device.create_shader_module(ShaderModuleDescriptor {
            label: Some(label),
            source: wgpu::ShaderSource::Wgsl(shader.into()),
        });
        let module = Arc::new(module);
        Self { module }
    }
}
#[macro_export]
macro_rules! shaders {
    ($device:expr,$path:expr,$label:expr) => {
        $crate::renderer::pipelines::shaders::MatrixShaders::from_string(
            $device,
            include_str!($path),
            $label,
        )
    };
}
