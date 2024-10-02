use wgpu::{BindGroupEntry, BindGroupLayoutEntry};

pub trait MatrixBindable {
    fn bind_layout_entry(index:u32) -> BindGroupLayoutEntry;

    fn bind_entry(&self) -> BindGroupEntry;
}