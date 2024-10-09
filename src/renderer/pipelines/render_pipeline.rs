use std::marker::PhantomData;

use super::bind_groups::bind_group_group::MatrixBindGroupableGroupable;
use super::device_queue::DeviceQueue;
use super::shaders::MatrixShaders;
use super::textures::MatrixTexture;
use super::vertecies::MatrixVertexBufferableGroupable;
use wgpu::{
    BlendState, ColorTargetState, ColorWrites, CompareFunction, DepthBiasState, DepthStencilState,
    PipelineCompilationOptions, RenderPipeline, StencilState, SurfaceConfiguration, TextureFormat,
};

pub struct MatrixRenderPipelineArgs<'a> {
    pub label: &'a str,
    pub device_queue: DeviceQueue,
    pub shaders: MatrixShaders,
    pub surface_config: &'a SurfaceConfiguration,
}

pub struct MatrixRenderPipeline<
    Vertex: MatrixVertexBufferableGroupable,
    BindGroupGroup: MatrixBindGroupableGroupable,
> {
    device_queue: DeviceQueue,
    pipeline: RenderPipeline,
    layouts: BindGroupGroup::Layouts,
    depth_texture: MatrixTexture,
    marker: PhantomData<(Vertex,)>,
}

impl<Vertex: MatrixVertexBufferableGroupable, BindGroupGroup: MatrixBindGroupableGroupable>
    MatrixRenderPipeline<Vertex, BindGroupGroup>
{
    pub fn new(args: MatrixRenderPipelineArgs) -> Self {
        let device_queue = args.device_queue;

        let layouts = BindGroupGroup::create_layouts(&device_queue);

        let pipeline_layout =
            device_queue
                .device()
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("matrix pipeline layout"),
                    bind_group_layouts: &BindGroupGroup::as_slice(&layouts),
                    push_constant_ranges: &[],
                });

        let depth_texture = MatrixTexture::create_depth_texture(&device_queue, args.surface_config);

        let pipeline =
            device_queue
                .device()
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some(args.label),
                    layout: Some(&pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: args.shaders.module(),
                        entry_point: "vs_main",
                        compilation_options: PipelineCompilationOptions::default(),
                        buffers: &Vertex::vertex_buffer_layouts(),
                    },
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: Some(wgpu::Face::Back),
                        unclipped_depth: false,
                        polygon_mode: wgpu::PolygonMode::Fill,
                        conservative: false,
                    },
                    depth_stencil: Some(DepthStencilState {
                        format: TextureFormat::Depth32Float,
                        depth_write_enabled: true,
                        depth_compare: CompareFunction::Less,
                        stencil: StencilState::default(),
                        bias: DepthBiasState::default(),
                    }),
                    multisample: wgpu::MultisampleState {
                        count: 1,
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: args.shaders.module(),
                        entry_point: "fs_main",
                        compilation_options: PipelineCompilationOptions::default(),
                        targets: &[Some(ColorTargetState {
                            format: args.surface_config.format,
                            blend: Some(BlendState::REPLACE),
                            write_mask: ColorWrites::ALL,
                        })],
                    }),
                    multiview: None,
                    cache: None,
                });

        Self {
            device_queue,
            pipeline,
            layouts,
            depth_texture,
            marker: PhantomData,
        }
    }

    pub(crate) fn setup_pass(&self, pass: &mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipeline);
    }

    pub(crate) fn setup_groups(
        &self,
        pass: &mut wgpu::RenderPass,
        groups: BindGroupGroup::Groups<'_>,
    ) {
        BindGroupGroup::setup_render_pass(pass, groups);
    }

    pub fn layouts(&self) -> &BindGroupGroup::Layouts {
        &self.layouts
    }

    pub(crate) fn setup_buffers(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        buffers: Vertex::Buffers<'_>,
    ) {
        Vertex::setup_pass(pass, buffers);
    }

    pub(crate) fn depth_texture(&self) -> &MatrixTexture {
        &self.depth_texture
    }

    pub(crate) fn configure_depth(
        &mut self,
        device_queue: &DeviceQueue,
        surface_config: &SurfaceConfiguration,
    ) {
        self.depth_texture = MatrixTexture::create_depth_texture(device_queue, surface_config)
    }
}
