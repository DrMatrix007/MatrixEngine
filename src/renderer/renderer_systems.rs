use std::{future::Future, sync::Arc};

use tokio::runtime::Builder;
use wgpu::{
    CommandEncoderDescriptor, Instance, InstanceDescriptor, Surface, SurfaceConfiguration,
    TextureViewDescriptor,
};
use winit::window::{Window, WindowId};

use crate::{
    engine::{
        component_iters::IntoWrapper,
        events::{MatrixEvent, MatrixEventable},
        query::{ReadC, ReadE, ReadR, ReadSystemID, WriteE, WriteR},
        transform::{Transform, TransformRaw},
    },
    renderer::pipelines::{device_queue::DeviceQueue, shaders::MatrixShaders, MatrixPipelineArgs},
};

use super::{
    atlas::Atlas,
    camera::{Camera, CameraUniform},
    pipelines::{
        bind_groups::bind_group::MatrixBindGroup,
        textures::MatrixTexture,
        vertecies::texture_vertex::{TextureVertex, TextureVertexBuffers},
        MatrixPipeline,
    },
    render_object::RenderObject,
};

pub struct RendererResource {
    pub(crate) device_queue: DeviceQueue,
    pub(crate) current_window_id: WindowId,
    pub(crate) surface: Surface<'static>,
    pub(crate) surface_config: SurfaceConfiguration,
    pub(crate) pipeline:
        MatrixPipeline<(TextureVertex, TransformRaw), (MatrixTexture, (CameraUniform,))>,
    pub(crate) atlas: Atlas,
    pub(crate) camera_uniform: (CameraUniform,),
    pub(crate) camera_binding_group: MatrixBindGroup<(CameraUniform,)>,
}

fn create_render_resource(window: &Window) -> RendererResource {
    let size = window.inner_size();
    let instance = Instance::new(InstanceDescriptor {
        backends: wgpu::Backends::PRIMARY,
        ..Default::default()
    });
    let surface = instance.create_surface(window).unwrap();
    let surface = unsafe { core::mem::transmute::<Surface<'_>, Surface<'static>>(surface) };

    let adapter = block_on(async {
        instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap()
    });

    let (device, queue) = block_on(async {
        adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    label: None,
                    memory_hints: Default::default(),
                },
                None, // Trace path
            )
            .await
            .unwrap()
    });

    let surface_caps = surface.get_capabilities(&adapter);

    let surface_format = surface_caps
        .formats
        .iter()
        .find(|f| f.is_srgb())
        .copied()
        .unwrap_or(surface_caps.formats[0]);
    let surface_config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::AutoNoVsync,
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };

    surface.configure(&device, &surface_config);

    let device = Arc::new(device);
    let queue = Arc::new(queue);

    let device_queue = DeviceQueue::new(device.clone(), queue.clone());

    let pipeline =
        MatrixPipeline::<_, (MatrixTexture, (CameraUniform,))>::new(MatrixPipelineArgs {
            shaders: MatrixShaders::new(&device_queue, include_str!("shaders.wgsl")),
            device_queue,
            surface_config: &surface_config,
        });

    let device_queue = DeviceQueue::new(device, queue);

    let camera_uniform = (CameraUniform::new(&device_queue),);

    let camera_binding_group = pipeline
        .layouts()
        .1
        .create_group(&device_queue, &camera_uniform);

    println!("created! - device name is {}", adapter.get_info().name);
    RendererResource {
        current_window_id: window.id(),
        camera_binding_group,
        camera_uniform,
        surface,
        surface_config,
        pipeline,
        atlas: Atlas::new(),
        device_queue,
    }
}
fn block_on<T>(future: impl Future<Output = T>) -> T {
    Builder::new_current_thread()
        .build()
        .unwrap()
        .block_on(future)
}

pub(crate) fn create_renderer_resource<CustomEvents: MatrixEventable>(
    renderer: &mut WriteR<RendererResource, CustomEvents>,
    window: &mut ReadR<Window>,
    event_writer: &mut WriteE<CustomEvents>,
    system_id: &mut ReadSystemID,
) {
    if let Some(window) = window.get() {
        renderer.unwrap_or_insert_with_and_notify(|| {
            event_writer
                .send(MatrixEvent::DestroySystem(**system_id))
                .unwrap();
            create_render_resource(window)
        });
    };
}

pub(crate) fn handle_resize<CustomEvents: MatrixEventable>(
    events: &mut ReadE<CustomEvents>,
    renderer: &mut WriteR<RendererResource, CustomEvents>,
    camera: &mut WriteR<Camera, CustomEvents>,
) {
    if let (Some(new_size), Some(renderer)) = (events.new_inner_size(), renderer.get_mut()) {
        if new_size.width * new_size.height > 0 {
            renderer.surface_config.width = new_size.width;
            renderer.surface_config.height = new_size.height;
            renderer
                .surface
                .configure(renderer.device_queue.device(), &renderer.surface_config);
            renderer
                .pipeline
                .configure_depth(&renderer.device_queue, &renderer.surface_config);
        }
        if let Some(camera) = camera.get_mut() {
            camera.aspect =
                renderer.surface_config.width as f32 / renderer.surface_config.height as f32
        }
    }
}

pub(crate) fn renderer_system<CustomEvents: MatrixEventable>(
    renderer: &mut WriteR<RendererResource, CustomEvents>,
    objects: &mut ReadC<RenderObject>,
    transforms: &mut ReadC<Transform>,
    camera: &mut ReadR<Camera>,
) {
    if let Some(renderer) = renderer.get_mut() {
        let output = if let Ok(output) = renderer.surface.get_current_texture() {
            output
        } else {
            return; // nothing to render
        };
        let view = output
            .texture
            .create_view(&TextureViewDescriptor::default());

        if let Some(camera) = camera.get() {
            renderer
                .camera_uniform
                .0
                .update_view_proj(&renderer.device_queue, camera);
        }

        let mut encoder =
            renderer
                .device_queue
                .device()
                .create_command_encoder(&CommandEncoderDescriptor {
                    label: Some("main render encoder"),
                });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
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
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: renderer.pipeline.depth_texture().view(),
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            {
                renderer.atlas.reset();

                // for (_, obj, transform) in objects
                //     .iter()
                //     .filter_map(|(e, obj)| transforms.get(e).map(move |t| (e, obj, t)))
                {
                    for (_, (obj, transform)) in (objects.iter(), transforms.iter()).into_wrapper()
                    {
                        renderer.atlas.write(
                            &renderer.device_queue,
                            obj,
                            transform,
                            &renderer.pipeline.layouts().0,
                        );
                    }
                }
                renderer.atlas.update_buffers(&renderer.device_queue);
                for instace in renderer.atlas.instances() {
                    renderer.pipeline.setup_pass(&mut render_pass);
                    renderer.pipeline.setup_groups(
                        &mut render_pass,
                        (instace.texture_group(), &renderer.camera_binding_group),
                    );
                    renderer.pipeline.setup_buffers(
                        &mut render_pass,
                        (
                            TextureVertexBuffers {
                                vertex_buffer: instace.vertex_buffer(),
                                index_buffer: instace.index_buffer(),
                            },
                            instace.instance_buffer(),
                        ),
                    );

                    render_pass.draw_indexed(0..instace.num_indices(), 0, 0..instace.instaces());
                }
                renderer.atlas.try_shrink(&renderer.device_queue);
            }
        }

        // submit will accept anything that implements IntoIter
        renderer
            .device_queue
            .queue()
            .submit(std::iter::once(encoder.finish()));
        output.present();
    }
}
