use std::marker::PhantomData;

use crate::engine::scenes::resources::Resource;
use wgpu::{
    DepthStencilState, Device, FragmentState, PipelineLayout, PrimitiveState, RenderPass,
    RenderPipeline, SurfaceConfiguration, VertexState,
};

use super::{
    buffers::{BufferContainer, BufferGroup, Bufferable, VertexBuffer},
    group_cluster::{BindGroupCluster, BindGroupLayoutContainerCluster},
    shaders::{MatrixShaders, ShaderConfig},
};

pub struct MatrixRenderPipelineArgs<'a> {
    pub device: &'a Device,
    pub shaders: MatrixShaders,
    pub shader_config: ShaderConfig,
    pub pipe_label: &'a str,
    pub group_label: &'a str,
    pub surface_config: &'a SurfaceConfiguration,
    pub primitive_state: PrimitiveState,
    pub depth_stencil: Option<DepthStencilState>,
}

pub struct MatrixRenderPipeline<B: BufferGroup, T: BindGroupCluster> {
    marker: PhantomData<(B, T)>,
    pipeline: RenderPipeline,
    layout: PipelineLayout,
    shaders: MatrixShaders,
}
impl<B: BufferGroup, T: BindGroupCluster> Resource for MatrixRenderPipeline<B, T> {}

impl<B: BufferGroup, T: BindGroupCluster> MatrixRenderPipeline<B, T> {
    pub fn apply_groups<'a>(&self, pass: &mut RenderPass<'a>, data: T::Args<'a>) {
        T::apply_to_pipeline(pass, data);
    }
    pub fn set_vertex_buffer<'a, Buff: Bufferable>(
        &self,
        pass: &mut RenderPass<'a>,
        buff: &'a VertexBuffer<Buff>,
        slot: u32,
    ) {
        pass.set_vertex_buffer(slot, buff.buffer().buffer().slice(..));
        if let Some(b) = buff.index_buffer() {
            pass.set_index_buffer(b.buffer().slice(..), wgpu::IndexFormat::Uint16);
        }
    }
    pub fn set_buffer<'a, Buff: Bufferable>(
        &self,
        pass: &mut RenderPass<'a>,
        buff: &'a BufferContainer<Buff>,
        slot: u32,
    ) {
        pass.set_vertex_buffer(slot, buff.buffer().slice(..));
    }
    pub fn pipeline(&self) -> &RenderPipeline {
        &self.pipeline
    }

    pub fn begin<'a: 'b, 'b>(&'a self, pass: &mut RenderPass<'b>) {
        pass.set_pipeline(&self.pipeline)
    }

    // pub(crate) fn apply_buffer<'a>(
    //     &self,
    //     pass: &mut RenderPass<'a>,
    //     buffer: &'a BufferContainer<Vertex>,
    // ) {
    //     pass.set_vertex_buffer(0, buffer.buffer().slice(..));
    // }
    // pub(crate) fn apply_index_buffer<'a>(
    //     &self,
    //     pass: &mut RenderPass<'a>,
    //     buffer: &'a BufferContainer<u16>,
    // ) {
    //     pass.set_index_buffer(buffer.buffer().slice(..), wgpu::IndexFormat::Uint16)
    // }

    pub fn new(
        MatrixRenderPipelineArgs {
            device,
            group_label,
            pipe_label,
            shader_config: shader_conf,
            shaders,
            surface_config,
            primitive_state,
            depth_stencil,
        }: MatrixRenderPipelineArgs<'_>,
    ) -> Self {
        let ls = T::create_bind_group_layouts(group_label, device);
        let ls = Box::new(ls.iter_groups().collect::<Vec<_>>());

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(pipe_label),
            bind_group_layouts: &ls,
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(pipe_label),
            layout: Some(&layout),
            vertex: VertexState {
                module: shaders.module(),
                buffers: &B::describe(),
                entry_point: shader_conf.vertex_entry(),
            },
            fragment: Some(FragmentState {
                module: shaders.module(),
                entry_point: shader_conf.fragment_entry(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: Some(wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: primitive_state,
            depth_stencil,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Self {
            marker: PhantomData,
            pipeline,
            shaders,
            layout,
        }
    }

    pub(crate) fn draw_indexed(
        &self,
        pass: &mut RenderPass<'_>,
        range: std::ops::Range<u32>,
        instances: std::ops::Range<u32>,
    ) {
        pass.draw_indexed(range, 0, instances);
    }
}
