use std::marker::PhantomData;

use bytemuck::{Pod, Zeroable};
use wgpu::BufferSlice;

use crate::arl::buffers::Buffer;

pub struct BufferedVec<T: Pod + Zeroable> {
    buffer: Buffer<T>,
    vec: Vec<T>,
    marker: PhantomData<T>,
}

impl<T: Pod + Zeroable> BufferedVec<T> {
    pub fn push(&mut self, value: T) {
        self.vec.push(value)
    }

    pub fn clear(&mut self) {
        self.vec.clear()
    }

    pub fn shrink(&mut self) {
        let new_size = self.vec.len().next_power_of_two();
        self.vec.shrink_to(new_size);
    }

    pub fn flush(&mut self) -> BufferSlice<'_> {
        let target_cap = self.vec.capacity() as u64;
        if self.buffer.raw().size() != target_cap {
            self.buffer.resize(target_cap, false);
        }
        self.buffer.write(&self.vec);

        self.buffer.raw().slice(0..(self.vec.len() as u64))
    }
}
