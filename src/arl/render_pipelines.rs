use std::marker::PhantomData;

use wgpu::SurfaceConfiguration;

use crate::arl::{
    device_queue::DeviceQueue, passable::Passable, shaders::Shaders, vertex::VertexGroup,
};

pub struct RenderPipelineArgs<'a, 'b> {
    pub shaders: &'a Shaders,
    pub surface_config: &'b SurfaceConfiguration,
}

pub struct RenderPipeline<VertexBuffers: VertexGroup> {
    pipeline: wgpu::RenderPipeline,
    _pipeline_layout: wgpu::PipelineLayout,
    marker: PhantomData<VertexBuffers>,
}

impl<VertexBuffers: VertexGroup> RenderPipeline<VertexBuffers> {
    pub fn new(label: &str, args: RenderPipelineArgs<'_, '_>, device_queue: &DeviceQueue) -> Self {
        let pipeline_layout =
            device_queue
                .device()
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some(format!("{label} layout").as_str()),
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                });

        let buffer_attrs = VertexBuffers::attrs();
        let buffer_attrs = VertexBuffers::desc(&buffer_attrs);
        let pipeline =
            device_queue
                .device()
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some(label),
                    layout: Some(&pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: args.shaders.raw(),
                        entry_point: Some(args.shaders.vertex_entry()),
                        compilation_options: Default::default(),
                        buffers: &buffer_attrs,
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: args.shaders.raw(),
                        entry_point: args.shaders.fragment_entry(),
                        targets: &[Some(wgpu::ColorTargetState {
                            format: args.surface_config.format,
                            blend: Some(wgpu::BlendState::REPLACE),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                    }),
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: Some(wgpu::Face::Back),
                        polygon_mode: wgpu::PolygonMode::Fill,
                        unclipped_depth: false,
                        conservative: false,
                    },
                    depth_stencil: None,
                    multisample: wgpu::MultisampleState {
                        count: 1,
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                    multiview: None,
                    cache: None,
                });

        Self {
            _pipeline_layout: pipeline_layout,
            pipeline,
            marker: PhantomData,
        }
    }

    pub fn raw(&self) -> &wgpu::RenderPipeline {
        &self.pipeline
    }
}

impl<VGroup: VertexGroup> Passable for RenderPipeline<VGroup> {
    fn apply<'a>(&self, pass: &mut wgpu::RenderPass<'a>) {
        pass.set_pipeline(&self.pipeline);
    }
}
