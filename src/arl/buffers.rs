use std::{
    marker::PhantomData,
    sync::{Arc, atomic::AtomicBool},
};

use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

use crate::arl::device_queue::DeviceQueue;

pub struct Buffer<T: Pod + Zeroable> {
    buffer: wgpu::Buffer,
    device_queue: DeviceQueue,
    label: String,
    marker: PhantomData<T>,
    usage: wgpu::BufferUsages,
    mapped: bool,
}

fn map_buffer(b: &wgpu::Buffer) {
    let atomic_send = Arc::new(AtomicBool::new(false));
    let atomic = atomic_send.clone();
    b.map_async(wgpu::MapMode::Write, .., move |_| {
        atomic_send.store(true, std::sync::atomic::Ordering::Release)
    });

    while atomic.load(std::sync::atomic::Ordering::Relaxed) {}
}

impl<T: Pod + Zeroable> Buffer<T> {
    pub fn new(
        label: &str,
        data: &[T],
        usage: wgpu::BufferUsages,
        device_queue: DeviceQueue,
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
            device_queue,
            label: label.to_string(),
            usage,
            mapped: false,
        }
    }

    pub fn new_mapped(
        label: &str,
        usage: wgpu::BufferUsages,
        device_queue: DeviceQueue,
        size: u64,
    ) -> Self {
        let buffer = device_queue
            .device()
            .create_buffer(&wgpu::BufferDescriptor {
                label: Some(label),
                mapped_at_creation: true,
                usage,
                size,
            });
        // map_buffer(&buffer);
        Self {
            buffer,
            marker: PhantomData,
            device_queue,
            label: label.to_string(),
            usage,
            mapped: true,
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

    pub fn write(&self, data: &[T]) {
        self.device_queue
            .queue()
            .write_buffer(&self.buffer, 0, bytemuck::cast_slice(data));
    }

    pub fn resize(&mut self, size: u64, copy_data: bool) {
        let size = size * core::mem::size_of::<T>() as u64;
        let new_buff = self
            .device_queue
            .device()
            .create_buffer(&wgpu::wgt::BufferDescriptor {
                label: Some(self.label.as_str()),
                size,
                usage: self.usage,
                mapped_at_creation: self.mapped,
            });
        // map_buffer(&new_buff);

        if copy_data {
            let mut encoder = self.device_queue.device().create_command_encoder(
                &wgpu::wgt::CommandEncoderDescriptor {
                    label: Some("arl::Buffer<T> resize command buffer"),
                },
            );

            encoder.copy_buffer_to_buffer(
                &self.buffer,
                0,
                &new_buff,
                0,
                new_buff.size().min(self.buffer.size()),
            );

            self.device_queue
                .queue()
                .submit(std::iter::once(encoder.finish()));
        }

        self.buffer = new_buff;
    }

    pub fn device_queue(&self) -> &DeviceQueue {
        &self.device_queue
    }
}
