use wgpu::{BindGroupEntry, BindGroupLayoutEntry};

use super::bind_group::{MatrixBindGroup, MatrixBindGroupable};

pub trait MatrixBindable {

    fn bind_layout_entry(binding: u32) -> BindGroupLayoutEntry;

    fn bind_entry(&self, binding: u32) -> BindGroupEntry;
}

