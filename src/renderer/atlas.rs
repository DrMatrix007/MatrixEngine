use std::{any::TypeId, collections::HashMap, sync::Arc};

use wgpu::{
    util::DeviceExt, Buffer, BufferAddress, BufferDescriptor, BufferUsages,
    CommandEncoderDescriptor,
};

use super::pipelines::device_queue::DeviceQueue;

#[derive(Debug)]
struct InstanceVector<T> {
    buffer: Arc<Buffer>,
    staging_buffer: Arc<Buffer>,
    size: usize,
    _marker: std::marker::PhantomData<T>,
}

impl<T: bytemuck::Pod> InstanceVector<T> {
    pub fn new(device_queue: &DeviceQueue, size: usize) -> Self {
        let buffer = device_queue
            .device()
            .create_buffer(&wgpu::BufferDescriptor {
                label: Some("Vector Buffer"),
                size: (size * std::mem::size_of::<T>()) as u64,
                usage: BufferUsages::VERTEX,
                mapped_at_creation: false,
            });
        let staging_buffer = device_queue.device().create_buffer(&BufferDescriptor {
            label: Some("Stage Buffer"),
            size: std::mem::size_of::<T>() as BufferAddress,
            usage: BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        Self {
            buffer: Arc::new(buffer),
            staging_buffer: Arc::new(staging_buffer),
            size,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    pub fn size(&self) -> usize {
        self.size
    }

    fn capacity(&self) -> usize {
        (self.buffer.size() as usize) / std::mem::size_of::<T>()
    }

    pub fn push(&mut self, device_queue: &DeviceQueue, element: T) {
        // If the current size equals the capacity, we need to grow the buffer
        if self.size == self.capacity() {
            // Double the capacity or set it to 1 if it's 0
            let new_size = if self.size == 0 { 1 } else { self.size * 2 };

            // Create a new buffer with the new size
            let new_buffer = device_queue
                .device()
                .create_buffer(&wgpu::BufferDescriptor {
                    label: Some("Wgpu Vector Buffer"),
                    size: (new_size * std::mem::size_of::<T>()) as u64,
                    usage: BufferUsages::VERTEX | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
                    mapped_at_creation: false,
                });

            // Copy the existing buffer contents to the new buffer
            let mut encoder =
                device_queue
                    .device()
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("Buffer Copy Encoder"),
                    });

            encoder.copy_buffer_to_buffer(
                &self.buffer,
                0,
                &new_buffer,
                0,
                (self.size * std::mem::size_of::<T>()) as u64,
            );

            device_queue.queue().submit(Some(encoder.finish()));

            // Replace the old buffer with the new buffer
            self.buffer = Arc::new(new_buffer);
        }

        // After ensuring enough space, we can add the new element
        device_queue.queue().write_buffer(
            &self.buffer,
            (self.size() * core::mem::size_of::<T>()) as BufferAddress,
            bytemuck::bytes_of(&element),
        );

        // Increment the size
        self.size += 1;
    }

    pub fn shrink_buffer(&mut self, device_queue: &DeviceQueue) {
        let mut new_size = self.capacity() / 2;
        if new_size < self.size() {
            return;
        }
        // Shrink the buffer by half until it fits the current number of elements
        while new_size > self.size {
            new_size /= 2;
        }

        let new_buffer = device_queue
            .device()
            .create_buffer(&wgpu::BufferDescriptor {
                label: Some("Shrinking Buffer"),
                size: (new_size * std::mem::size_of::<T>()) as u64,
                usage: BufferUsages::VERTEX | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
                mapped_at_creation: false,
            });
        let mut encoder = device_queue
            .device()
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("shrink command encoder"),
            });

        encoder.copy_buffer_to_buffer(
            &self.buffer,
            0,
            &new_buffer,
            0,
            (self.size() * core::mem::size_of::<T>()) as BufferAddress,
        );

        device_queue.queue().submit(Some(encoder.finish()));

        self.buffer = Arc::new(new_buffer);
    }
}

pub struct InstacedType {
    texture_path: String,
    model: TypeId,
}

pub(crate) struct Atlas {
    data: HashMap<InstacedType, InstanceVector<()>>,
}
