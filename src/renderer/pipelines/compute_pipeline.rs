use super::bind_groups::bind_group_group::MatrixBindGroupableGroupable;
use super::device_queue::DeviceQueue;
use super::shaders::MatrixShaders;
use std::marker::PhantomData;
use wgpu::{
    ComputePipeline, PipelineCompilationOptions, PipelineLayoutDescriptor,
};

pub struct MatrixComputePipelineArgs<'a> {
    pub label: &'a str,
    pub device_queue: DeviceQueue,
    pub shaders: MatrixShaders,
}

pub struct MatrixComputePipeline<BindGroupGroup: MatrixBindGroupableGroupable> {
    device_queue: DeviceQueue,
    pipeline: ComputePipeline,
    layouts: BindGroupGroup::Layouts,
    marker: PhantomData<BindGroupGroup>,
}

impl<BindGroupGroup: MatrixBindGroupableGroupable> MatrixComputePipeline<BindGroupGroup> {
    pub fn new(args: MatrixComputePipelineArgs) -> Self {
        let device_queue = args.device_queue;

        // Create the bind group layouts
        let layouts = BindGroupGroup::create_layouts(&device_queue);

        // Create the pipeline layout
        let pipeline_layout =
            device_queue
                .device()
                .create_pipeline_layout(&PipelineLayoutDescriptor {
                    label: Some("matrix compute pipeline layout"),
                    bind_group_layouts: &BindGroupGroup::as_slice(&layouts),
                    push_constant_ranges: &[],
                });

        // Create the compute pipeline
        let pipeline =
            device_queue
                .device()
                .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: Some(args.label),
                    layout: Some(&pipeline_layout),
                    module: args.shaders.module(),
                    entry_point: "cs_main", // Entry point for compute shader,
                    cache: None,
                    compilation_options: PipelineCompilationOptions::default(),
                });

        Self {
            device_queue,
            pipeline,
            layouts,
            marker: PhantomData,
        }
    }

    pub(crate) fn setup_pass(&self, pass: &mut wgpu::ComputePass<'_>) {
        pass.set_pipeline(&self.pipeline);
    }

    pub(crate) fn setup_groups(
        &self,
        pass: &mut wgpu::ComputePass<'_>,
        groups: BindGroupGroup::Groups<'_>,
    ) {
        BindGroupGroup::setup_compute_pass(pass, groups);
    }

    pub fn layouts(&self) -> &BindGroupGroup::Layouts {
        &self.layouts
    }
}
