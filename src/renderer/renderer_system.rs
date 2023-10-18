use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use wgpu::{CommandEncoderDescriptor, Device, Queue, Surface, SurfaceError, TextureViewDescriptor};
use winit::window::Window;

use crate::engine::{
    events::event_registry::EventRegistry,
    scenes::resources::Resource,
    systems::{QuerySystem, SystemControlFlow},
};

struct FpsCounter {
    last: Instant,
}

impl FpsCounter {
    pub fn new() -> Self {
        Self {
            last: Instant::now(),
        }
    }
    pub fn capture(&mut self) -> Duration {
        let now = Instant::now();
        let duration = now - self.last;
        self.last = now;

        duration
    }
    pub fn capture_as_fps(&mut self) -> f64 {
        let d = self.capture();
        1. / d.as_secs_f64()
    }
}

impl Resource for Window {}

#[derive(Debug, Clone)]
pub struct DeviceQueue {
    device: Arc<Device>,
    queue: Arc<Queue>,
}
impl DeviceQueue {
    fn new(device: Device, queue: Queue) -> Self {
        Self {
            device: Arc::new(device),
            queue: Arc::new(queue),
        }
    }
    pub fn device(&self) -> &Arc<Device> {
        &self.device
    }
    pub fn queue(&self) -> &Arc<Queue> {
        &self.queue
    }
}

pub struct RendererSystem {
    window: Window,
    surface: Surface,
    device: DeviceQueue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    fps: FpsCounter,
}

impl RendererSystem {
    pub fn new(window: Window) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::VULKAN,
            dx12_shader_compiler: Default::default(),
        });

        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let (adapter, queue, device) = tokio::runtime::Builder::new_current_thread()
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
                    .request_device(
                        &wgpu::DeviceDescriptor {
                            features: wgpu::Features::empty(),
                            // WebGL doesn't support all of wgpu's features, so if
                            // we're building for the web we'll have to disable some.
                            limits: if cfg!(target_arch = "wasm32") {
                                wgpu::Limits::downlevel_webgl2_defaults()
                            } else {
                                wgpu::Limits::default()
                            },
                            label: None,
                        },
                        None, // Trace path
                    )
                    .await
                    .unwrap();
                (adapter, queue, device)
            });

        let device_queue = DeviceQueue::new(device, queue);

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device_queue.device, &config);

        Self {
            window,
            config,
            device: device_queue,
            size,
            surface,
            fps: FpsCounter::new(),
        }
    }

    fn render(&mut self) -> Result<(), SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .device()
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("main command encoder"),
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
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
        }

        self.device.queue.submit(core::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}

impl QuerySystem for RendererSystem {
    type Query = ();

    fn run(
        &mut self,
        events: &EventRegistry,
        args: &mut Self::Query,
    ) -> crate::engine::systems::SystemControlFlow {
        println!("fps: {}", self.fps.capture_as_fps());

        let _ = self.render();

        let window_events = events.get_window_events(self.window.id());

        let new_size = window_events.size();
        if self.size != new_size && new_size.width != 0 && new_size.height != 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.size = new_size;
            self.surface.configure(&self.device.device, &self.config);
        }

        if window_events.should_close() {
            return SystemControlFlow::Quit;
        }
        // spin_sleep::sleep(Duration::from_secs_f64(0.3));
        crate::engine::systems::SystemControlFlow::Continue
    }
}

// impl QuerySystem for RendererSystem {
//     type Query = ReadR<Window>;

//     fn run(
//         &mut self,
//         args: &mut <Self::Query as crate::engine::systems::query::Query<
//             crate::engine::systems::query::ComponentQueryArgs,
//         >>::Target,
//     ) {
//     }
// }
