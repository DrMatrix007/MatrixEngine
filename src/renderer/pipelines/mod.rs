use std::marker::PhantomData;

use bind_groups::bind_group_group::MatrixBindGroupableGroupable;
use device_queue::DeviceQueue;
use shaders::MatrixShaders;
use vertecies::Vertexable;
use wgpu::{
    BlendState, ColorTargetState, ColorWrites, PipelineCompilationOptions, RenderPipeline,
    SurfaceConfiguration,
};

pub mod bind_groups;
pub mod device_queue;
pub mod shaders;
pub mod textures;
pub mod vertecies;

pub struct MatrixPipelineArgs<'a, Vertex: Vertexable> {
    pub device_queue: DeviceQueue,
    pub shaders: MatrixShaders<Vertex>,
    pub surface_config: &'a SurfaceConfiguration,
}

pub struct MatrixPipeline<Vertex: Vertexable, BindGroupGroup: MatrixBindGroupableGroupable> {
    device_queue: DeviceQueue,
    pipeline: RenderPipeline,
    layouts: BindGroupGroup::Layouts,
    marker: PhantomData<(Vertex,)>,
}

impl<Vertex: Vertexable, BindGroupGroup: MatrixBindGroupableGroupable>
    MatrixPipeline<Vertex, BindGroupGroup>
{
    pub fn new(args: MatrixPipelineArgs<Vertex>) -> Self {
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

        let pipeline =
            device_queue
                .device()
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("matrix pipeline layout"),
                    layout: Some(&pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: args.shaders.module(),
                        entry_point: "vs_main",
                        compilation_options: PipelineCompilationOptions::default(),
                        buffers: &[Vertex::vertex_buffer_layout()],
                    },
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleStrip,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: Some(wgpu::Face::Back),
                        unclipped_depth: false,
                        polygon_mode: wgpu::PolygonMode::Fill,
                        conservative: false,
                    },
                    depth_stencil: None,
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
        BindGroupGroup::setup_pass(pass, groups);
    }
}
