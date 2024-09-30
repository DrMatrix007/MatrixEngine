use std::future::Future;

use tokio::runtime::{Builder, Runtime};
use wgpu::{
    CommandEncoderDescriptor, Device, Instance, InstanceDescriptor, Queue, Surface,
    SurfaceConfiguration, TextureViewDescriptor,
};
use winit::window::{Window, WindowId};

use crate::engine::{
    events::{MatrixEvent, MatrixEventable},
    query::{ReadE, ReadR, ReadSystemID, WriteE, WriteR},
};

pub struct RendererResource {
    pub(crate) device: Device,
    pub(crate) queue: Queue,
    pub(crate) current_window_id: WindowId,
    pub(crate) surface: Surface<'static>,
    pub(crate) surface_config: SurfaceConfiguration,
}

fn create_render_resource<CustomEvents: MatrixEventable>(
    window: &Window,
    event_writer: &WriteE<CustomEvents>,
    system_id: &ReadSystemID,
) -> RendererResource {
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
    // Shader code in this tutorial assumes an sRGB surface texture. Using a different
    // one will result in all the colors coming out darker. If you want to support non
    // sRGB surfaces, you'll need to account for that when drawing to the frame.
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
        present_mode: surface_caps.present_modes[0],
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };

    surface.configure(&device, &surface_config);

    event_writer
        .send(MatrixEvent::DestroySystem(**system_id))
        .unwrap();
    println!("created! - device name is {}", adapter.get_info().name);
    RendererResource {
        current_window_id: window.id(),
        device,
        queue,
        surface,
        surface_config,
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
            create_render_resource(window, event_writer, system_id)
        });
    };
}

pub(crate) fn handle_resize<CustomEvents: MatrixEventable>(
    events: &mut ReadE<CustomEvents>,
    renderer: &mut WriteR<RendererResource, CustomEvents>,
) {
    if let (Some(new_size), Some(render)) = (events.new_inner_size(), renderer.get_mut()) {
        if new_size.width * new_size.height > 0 {
            render.surface_config.width = new_size.width;
            render.surface_config.height = new_size.height;
            render
                .surface
                .configure(&render.device, &render.surface_config);
        }
    }
}

pub(crate) fn renderer_system<CustomEvents: MatrixEventable>(
    renderer: &mut WriteR<RendererResource, CustomEvents>,
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

        let mut encoder = renderer
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("main render encoder"),
            });
        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
        }

        // submit will accept anything that implements IntoIter
        renderer.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
}
