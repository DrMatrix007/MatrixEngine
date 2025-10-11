use std::{marker::PhantomData, num::NonZeroUsize};

use bytemuck::{Pod, Zeroable};
use wgpu::BufferSlice;

use crate::arl::{buffers::Buffer, device_queue::DeviceQueue};

pub struct BufferedVec<T: Pod + Zeroable> {
    buffer: Buffer<T>,
    vec: Vec<T>,
    marker: PhantomData<T>,
}

impl<T: Pod + Zeroable> BufferedVec<T> {
    pub fn new(label: &str, usage: wgpu::BufferUsages, device_queue: DeviceQueue) -> Self {
        Self {
            buffer: Buffer::new(label, &[], usage, device_queue),
            vec: Vec::with_capacity(0),
            marker: PhantomData,
        }
    }

    pub fn push(&mut self, value: T) {
        self.vec.push(value)
    }

    pub fn clear(&mut self) {
        self.vec.clear()
    }

    fn shrink(&mut self) {
        let new_size = self.vec.len().next_power_of_two();
        self.vec.shrink_to(new_size);
    }

    pub fn flush(&mut self) {
        self.shrink();
        let target_cap = self.vec.capacity() as u64;
        if self.buffer.raw().size() != target_cap {
            self.buffer.resize(target_cap, false);
        }
        self.buffer.write(&self.vec);
    }

    pub fn buffer(&self) -> &Buffer<T> {
        &self.buffer
    }

    pub fn curr_size(&self) -> u32 {
        self.vec.len() as _
    }

    pub fn curr_data(&self) -> &[T] {
        &self.vec
    }
}
