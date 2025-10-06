use wgpu::util::DeviceExt;

use crate::arl::device_queue::DeviceQueue;

pub struct Buffer {
    buffer: wgpu::Buffer,
}

impl Buffer {
    pub fn new(
        label: &str,
        data: &[u8],
        usage: wgpu::BufferUsages,
        device_queue: DeviceQueue,
    ) -> Self {
        Self {
            buffer: device_queue
                .device()
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: data,
                    usage,
                }),
        }
    }

    pub fn raw(&self) -> &wgpu::Buffer {
        &self.buffer
    }
}
