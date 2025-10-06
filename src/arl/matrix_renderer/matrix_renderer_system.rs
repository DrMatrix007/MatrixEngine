use std::sync::Arc;

use wgpu::{
    Instance, RenderPassColorAttachment, Surface, SurfaceConfiguration, SurfaceError,
    TextureViewDescriptor,
};
use winit::{event::WindowEvent, window::Window};

use crate::{
    arl::{
        device_queue::DeviceQueue,
        matrix_renderer::matrix_vertex::MatrixVertex,
        render_pipelines::{RenderPipeline, RenderPipelineArgs},
        shaders::{Shaders, ShadersArgs},
    },
    engine::{query::Res, system_registries::Stage},
};

pub struct MatrixRenderInstance {
    device_queue: DeviceQueue,
    wgpu_instance: Instance,
    surface: Surface<'static>,
    surface_config: SurfaceConfiguration,
    is_surface_updated: bool,
    _shaders: Shaders,
    pipeline: RenderPipeline<MatrixVertex>,
}

impl MatrixRenderInstance {
    pub fn device_queue(&self) -> &DeviceQueue {
        &self.device_queue
    }

    pub fn surface_config(&self) -> &SurfaceConfiguration {
        &self.surface_config
    }

    pub fn surface_config_mut(&mut self) -> &mut SurfaceConfiguration {
        &mut self.surface_config
    }

    pub fn surface(&self) -> &Surface<'static> {
        &self.surface
    }

    pub fn surface_mut(&mut self) -> &mut Surface<'static> {
        &mut self.surface
    }

    pub fn wgpu_instance(&self) -> &Instance {
        &self.wgpu_instance
    }
}

pub fn matrix_renderer(
    stage: &mut Stage,
    window: &mut Res<Window>,
    instance: &mut Res<MatrixRenderInstance>,
) {
    let window = match (stage, window.as_mut()) {
        (Stage::Render(id), maybe_window) => {
            if let Some(window) = maybe_window {
                if *id != window.id() {
                    return;
                } else {
                    window
                }
            } else {
                return;
            }
        }
        _ => {
            panic!("this should be run in StageDescriptor::Render!");
        }
    };
    let instance = match instance.as_mut() {
        Some(instance) => instance,
        None => return,
    };

    if !instance.is_surface_updated {
        instance.is_surface_updated = true;
        instance
            .surface
            .configure(instance.device_queue().device(), instance.surface_config());
    }

    let output = match instance.surface.get_current_texture() {
        Ok(data) => data,
        Err(SurfaceError::Outdated | SurfaceError::Lost) => {
            instance.is_surface_updated = false;
            return;
        }
        err => panic!("surface error: {err:?}"),
    };

    let view = output
        .texture
        .create_view(&TextureViewDescriptor::default());

    let mut encoder = instance.device_queue().device().create_command_encoder(
        &wgpu::wgt::CommandEncoderDescriptor {
            label: Some("Matrix Render Encoder"),
        },
    );

    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        render_pass.set_pipeline(instance.pipeline.raw()); // 2.
        render_pass.draw(0..3, 0..1); // 3.
    }

    instance
        .device_queue()
        .queue()
        .submit(std::iter::once(encoder.finish()));

    output.present();

    window.request_redraw();
}

pub fn create_matrix_instance(window: &mut Res<Window>, res: &mut Res<MatrixRenderInstance>) {
    let window = match window.as_mut() {
        Some(window) => window,
        None => return,
    };

    let size = window.inner_size();

    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
        #[cfg(not(target_arch = "wasm32"))]
        backends: wgpu::Backends::PRIMARY,
        #[cfg(target_arch = "wasm32")]
        backends: wgpu::Backends::GL,
        ..Default::default()
    });

    let surface = unsafe {
        core::mem::transmute::<Surface<'_>, Surface<'static>>(
            instance.create_surface(window).unwrap(),
        )
    };

    let (adapter, device, queue) = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap()
        .block_on(async {
            let adapter = instance
                .request_adapter(&wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::default(),
                    compatible_surface: Some(&surface),
                    force_fallback_adapter: false,
                })
                .await
                .unwrap();
            let (device, queue) = adapter
                .request_device(&wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    experimental_features: wgpu::ExperimentalFeatures::disabled(),
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web we'll have to disable some.
                    required_limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    memory_hints: Default::default(),
                    trace: wgpu::Trace::Off,
                })
                .await
                .unwrap();
            (adapter, device, queue)
        });

    let surface_caps = surface.get_capabilities(&adapter);

    let surface_format = surface_caps
        .formats
        .iter()
        .find(|f| f.is_srgb())
        .copied()
        .unwrap_or(surface_caps.formats[0]);
    let mut config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width,
        height: size.height,
        present_mode: surface_caps.present_modes[0],
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };

    config.present_mode = wgpu::PresentMode::AutoNoVsync;

    let device_queue = DeviceQueue::new(Arc::new(device), Arc::new(queue));

    let shaders = Shaders::new(
        "matrix shaders",
        ShadersArgs {
            fragment_entry: Some("fs_main".into()),
            vertex_entry: "vs_main".into(),
            shaders: include_str!("shaders.wgsl").into(),
        },
        &device_queue,
    );

    let pipeline = RenderPipeline::new(
        "matrix pipeline",
        RenderPipelineArgs {
            shaders: &shaders,
            surface_config: &config,
        },
        &device_queue,
    );

    res.replace(MatrixRenderInstance {
        device_queue,
        wgpu_instance: instance,
        surface,
        surface_config: config,
        is_surface_updated: false,
        _shaders: shaders,
        pipeline,
    });
}

pub fn update_surface_size(stage: &mut Stage, res: &mut Res<MatrixRenderInstance>) {
    let event = match stage {
        Stage::WindowEvent(event) => event,
        _ => {
            panic!("this should run as StageDescriptor::WindowEvent!")
        }
    };

    let res = match res.as_mut() {
        Some(res) => res,
        None => return,
    };

    if let WindowEvent::Resized(size) = event
        && size.width > 0
        && size.height > 0
    {
        res.is_surface_updated = false;
        res.surface_config.width = size.width;
        res.surface_config.height = size.height;
    }
}
