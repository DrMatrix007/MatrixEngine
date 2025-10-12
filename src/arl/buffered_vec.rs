use std::marker::PhantomData;

use bytemuck::{Pod, Zeroable};

use crate::arl::{buffers::Buffer, device_queue::DeviceQueue};

pub struct BufferedVec<T: Pod + Zeroable> {
    buffer: Buffer<T>,
    vec: Vec<T>,
    marker: PhantomData<T>,
}

impl<T: Pod + Zeroable> BufferedVec<T> {
    pub fn new(label: &str, usage: wgpu::BufferUsages, device_queue: DeviceQueue) -> Self {
        Self {
            // buffer: Buffer::new_mapped(label, usage, device_queue, 0),
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
        if self.buffer.len() != target_cap {
            println!("????");
            self.buffer.resize(target_cap, false);
        }

        self.buffer.device_queue().queue().write_buffer(
            self.buffer.raw(),
            0,
            bytemuck::cast_slice(&self.vec),
        );

        // let bounds = 0..(self.vec.len() as u64 * core::mem::size_of::<T>() as u64);
        // if !resized {
        //     self.buffer.raw().map_async(
        //         wgpu::MapMode::Write,
        //         bounds.clone(),
        //         |res| if res.is_ok() {},
        //     );

        //     self.buffer
        //         .device_queue()
        //         .device()
        //         .poll(wgpu::wgt::PollType::Poll)
        //         .unwrap();
        // }

        // let mut map = self.buffer.raw().get_mapped_range_mut(bounds);

        // map.as_mut()
        //     .copy_from_slice(bytemuck::cast_slice(&self.vec));

        // drop(map);

        // self.buffer.raw().unmap();
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
