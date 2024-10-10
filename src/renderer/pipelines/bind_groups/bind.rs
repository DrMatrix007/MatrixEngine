use wgpu::{BindGroupEntry, BindGroupLayoutEntry};

use super::bind_group::{MatrixBindGroupable};

pub trait MatrixBindable {
    fn bind_layout_entry(binding: u32) -> BindGroupLayoutEntry;

    fn bind_entry(&self, binding: u32) -> BindGroupEntry;
}

impl<T: MatrixBindable> MatrixBindGroupable for T {
    fn create_group_layout(
        device_queue: &crate::renderer::pipelines::device_queue::DeviceQueue,
    ) -> wgpu::BindGroupLayout {
        device_queue
            .device()
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("matrix bind group single layout"),
                entries: &[T::bind_layout_entry(0)],
            })
    }

    fn create_group(
        &self,
        device_queue: &crate::renderer::pipelines::device_queue::DeviceQueue,
        layout: &super::bind_group::MatrixBindGroupLayout<Self>,
    ) -> wgpu::BindGroup
    where
        Self: Sized,
    {
        device_queue
            .device()
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("matrix bind group single layout"),
                layout: layout.layout(),
                entries: &[self.bind_entry(0)],
            })
    }
}
