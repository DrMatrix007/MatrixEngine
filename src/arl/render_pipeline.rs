use std::marker::PhantomData;

use wgpu::SurfaceConfiguration;

use crate::arl::{
    atlas::Atlas,
    bind_groups::bind_group_group::BindGroupableGroup,
    device_queue::DeviceQueue,
    models::ModelIDable,
    shaders::Shaders,
    vertex::vertexable::{VertexIndexer, VertexableGroup},
};

pub struct RenderPipelineArgs<'a, 'b> {
    pub shaders: &'a Shaders,
    pub surface_config: &'b SurfaceConfiguration,
}

pub struct RenderPipeline<
    ModelID: ModelIDable,
    Indexer: VertexIndexer,
    VertexGroup: VertexableGroup,
    BindGroups: BindGroupableGroup,
> {
    pipeline: wgpu::RenderPipeline,
    _pipeline_layout: wgpu::PipelineLayout,
    atlas: Atlas<ModelID, Indexer, VertexGroup, BindGroups>,
    marker: PhantomData<(VertexGroup, BindGroups)>,
}

impl<
    ModelID: ModelIDable,
    Indexer: VertexIndexer,
    VertexGroup: VertexableGroup,
    BindGroups: BindGroupableGroup,
> RenderPipeline<ModelID, Indexer, VertexGroup, BindGroups>
{
    pub fn new(label: &str, args: RenderPipelineArgs<'_, '_>, device_queue: &DeviceQueue) -> Self {
        let atlas = Atlas::new(device_queue);
        let pipeline_layout =
            device_queue
                .device()
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some(format!("{label} layout").as_str()),
                    bind_group_layouts: atlas.layout_desc().as_ref(),
                    push_constant_ranges: &[],
                });

        let buffer_attrs = VertexGroup::attrs();
        let buffer_attrs = VertexGroup::desc(&buffer_attrs);
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
            atlas,
            marker: PhantomData,
        }
    }

    pub fn raw(&self) -> &wgpu::RenderPipeline {
        &self.pipeline
    }

    pub fn atlas(&self) -> &Atlas<ModelID, Indexer, VertexGroup, BindGroups> {
        &self.atlas
    }

    pub fn atlas_mut(&mut self) -> &mut Atlas<ModelID, Indexer, VertexGroup, BindGroups> {
        &mut self.atlas
    }
}
