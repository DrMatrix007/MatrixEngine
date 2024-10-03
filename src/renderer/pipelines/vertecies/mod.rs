pub mod texture_vertex;

use wgpu::{RenderPass, VertexAttribute, VertexBufferLayout};

use crate::impl_all;

pub trait MatrixVertexBufferable: 'static {
    type Buffer<'a>;
    fn setup_pass(pass: &mut RenderPass<'_>, index: u32, buffer: Self::Buffer<'_>);

    const ATTRS: &[VertexAttribute];
    fn vertex_buffer_layout() -> VertexBufferLayout<'static>;
}

pub trait MatrixVertexBufferableGroupable {
    fn vertex_buffer_layouts() -> Vec<VertexBufferLayout<'static>>;

    type Buffers<'a>;

    fn setup_pass(pass: &mut RenderPass<'_>, buffers: Self::Buffers<'_>);
}

macro_rules! impl_group {
    ($($t:tt)*) => {
        impl<$($t:MatrixVertexBufferable,)*> MatrixVertexBufferableGroupable for ($($t,)*) {
            fn vertex_buffer_layouts() -> Vec<VertexBufferLayout<'static>>{
                vec![$($t::vertex_buffer_layout(),)*]
            }

            type Buffers<'a> = ($($t::Buffer<'a>,)*);

            #[allow(non_snake_case)]
            fn setup_pass(pass: &mut RenderPass<'_>, buffers: Self::Buffers<'_>) {
                let ($($t,)*) = buffers;
                let mut i = 0;
                $(<$t>::setup_pass(pass,{i+=1;i-1},$t);)*
            }

        }
    };
}

impl_all!(impl_group);
