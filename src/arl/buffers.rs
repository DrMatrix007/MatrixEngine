use std::marker::PhantomData;

use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

use crate::arl::device_queue::DeviceQueue;

pub struct Buffer<T: Pod + Zeroable> {
    buffer: wgpu::Buffer,
    marker: PhantomData<T>,
}

impl<T: Pod + Zeroable> Buffer<T> {
    pub fn new(
        label: &str,
        data: &[T],
        usage: wgpu::BufferUsages,
        device_queue: &DeviceQueue,
    ) -> Self {
        Self {
            buffer: device_queue
                .device()
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: bytemuck::cast_slice(data),
                    usage,
                }),
            marker: PhantomData,
        }
    }

    pub fn raw(&self) -> &wgpu::Buffer {
        &self.buffer
    }

    pub fn len(&self) -> u64 {
        self.buffer.size() / core::mem::size_of::<T>() as u64
    }
    
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
