use wgpu::{BindGroupEntry, BindGroupLayoutEntry};

pub trait MatrixBindable {
    fn bind_layout_entry() -> BindGroupLayoutEntry;

    fn bind_entry(&self) -> BindGroupEntry;
}