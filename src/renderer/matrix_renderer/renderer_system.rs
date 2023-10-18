use std::{collections::VecDeque, sync::Arc};

use crate::{
    engine::{
        events::event_registry::EventRegistry,
        scenes::resources::Resource,
        systems::{
            query::{components::ReadC, resources::WriteR},
            QuerySystem, SystemControlFlow,
        },
    },
    renderer::pipelines::{
        buffers::Vertex,
        group_layout_manager::BindGroupLayoutManager,
        instance_manager::InstanceManager,
        matrix_render_pipeline::{MatrixRenderPipeline, MatrixRenderPipelineArgs},
        shaders::ShaderConfig,
        texture::MatrixTexture,
        transform::{InstanceTransform, Transform},
    },
    shaders,
};

use wgpu::{
    Color, CommandEncoderDescriptor, Device, Operations, Queue, Surface, SurfaceConfiguration,
    SurfaceError,
};
use winit::{dpi::PhysicalSize, window::Window};

use super::{
    camera::{CameraResource, CameraUniform},
    render_object::RenderObject,
};

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

pub struct RendererResourceArgs {
    pub window: Window,
    pub background_color: Color,
}

pub struct RendererResource {
    surface: Surface,
    window: Window,
    device: DeviceQueue,
    config: SurfaceConfiguration,
    background_color: Color,
    group_layout_manager: BindGroupLayoutManager,
    instance_manager: InstanceManager,
    depth_texture: MatrixTexture,
    command_buffer_queue: VecDeque<(wgpu::CommandBuffer, std::boxed::Box<wgpu::SurfaceTexture>)>,
}

impl RendererResource {
    pub fn new(args: RendererResourceArgs) -> Self {
        let size = args.window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::VULKAN,
            dx12_shader_compiler: Default::default(),
        });

        let surface = unsafe { instance.create_surface(&args.window) }.unwrap();

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
        surface.configure(&device, &config);
        let device = DeviceQueue::new(device, queue);
        Self {
            depth_texture: MatrixTexture::create_depth_texture(&device, &config),
            config,
            surface,
            background_color: args.background_color,
            group_layout_manager: BindGroupLayoutManager::new(device.clone()),
            instance_manager: InstanceManager::new(device.clone()),
            command_buffer_queue: VecDeque::new(),
            device,
            window: args.window,
        }
    }

    fn resize(&mut self, size: &PhysicalSize<u32>) {
        if size.width > 0 || size.height > 0 {
            self.config.width = size.width;
            self.config.height = size.height;
            self.surface
                .configure(&self.device().device(), &self.config);

            self.depth_texture = MatrixTexture::create_depth_texture(&self.device, &self.config)
        }
    }

    pub fn device(&self) -> &DeviceQueue {
        &self.device
    }

    pub fn group_layout_manager_mut(&mut self) -> &mut BindGroupLayoutManager {
        &mut self.group_layout_manager
    }

    pub fn instance_manager_mut(&mut self) -> &mut InstanceManager {
        &mut self.instance_manager
    }

    pub fn window(&self) -> &Window {
        &self.window
    }
}

impl Resource for RendererResource {}

pub struct RendererSystem;

impl QuerySystem for RendererSystem {
    type Query = (
        (
            WriteR<RendererResource>,
            WriteR<MainPipeline>,
            WriteR<CameraResource>,
        ),
        (ReadC<RenderObject>, ReadC<Transform>),
    );

    fn run(
        &mut self,
        events: &EventRegistry,
        (
            (render_resource, main_pipeline, camera_resource),
            (render_objects, transforms),
        ): &mut Self::Query,
    ) -> SystemControlFlow {
        let render_resource = match render_resource.get_mut() {
            Some(data) => data,
            None => return SystemControlFlow::Continue,
        };

        // for (buff, output) in render_resource.command_buffer_queue.drain(..) {
        //     render_resource
        //         .device()
        //         .queue()
        //         .submit(std::iter::once(buff));
        //     (*output).present();
        //     println!("fuck");
        // }
        let main_pipeline = main_pipeline.get_or_insert_with(|| {
            MainPipeline::new(MatrixRenderPipelineArgs {
                device: &render_resource.device().device(),
                shaders: shaders!(
                    &render_resource.device().device(),
                    "shaders.wgsl",
                    "main shaders"
                ),
                shader_config: ShaderConfig {
                    fragment_main: "f_main".to_owned(),
                    vertex_main: "v_main".to_owned(),
                },
                pipe_label: "main pipeline",
                group_label: "main groups",
                surface_config: &render_resource.config,
                primitive_state: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: MatrixTexture::DEPTH_FORMAT,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::LessEqual,
                    stencil: Default::default(),
                    bias: Default::default(),
                }),
            })
        });
        let events = events.get_window_events(render_resource.window.id());
        {
            let new_size = events.size();
            if new_size.width != render_resource.config.width
                || new_size.height != render_resource.config.height
            {
                render_resource.resize(&new_size);
            }
        }

        let camera_resource =
            camera_resource.get_or_insert_with(|| CameraResource::new(render_resource));

        camera_resource.update_buffer(render_resource.device().queue());
        {
            let s = render_resource.window.inner_size();
            camera_resource.camera_mut().prespective.aspect = s.width as f32 / s.height as f32;
        }
        let current = render_resource.surface.get_current_texture();
        if let Ok(output) = current {
            let view = output.texture.create_view(&Default::default());

            let mut encoder = render_resource.device().device().create_command_encoder(
                &CommandEncoderDescriptor {
                    label: Some("main render encoder"),
                },
            );
            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("main render pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: Operations {
                            load: wgpu::LoadOp::Clear(render_resource.background_color),
                            store: true,
                        },
                    })],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: render_resource.depth_texture.view(),
                        depth_ops: Some(Operations {
                            load: wgpu::LoadOp::Clear(1.0),
                            store: true,
                        }),
                        stencil_ops: None,
                    }),
                });

                main_pipeline.begin(&mut pass);
                // objects.iter().for_each(|(_, data, trans)| {
                //     render_resource.instance_manager.register_object(
                //         data,
                //         trans,
                //         &mut render_resource.group_layout_manager,
                //     );
                //     // main_pipeline
                //     //     .apply_groups(&mut pass, (data.texture_group(), camera_resource.group()));

                //     // main_pipeline.apply_index_buffer(&mut pass, data.index_buffer());
                //     // main_pipeline.apply_buffer(&mut pass, data.buffer());

                //     // main_pipeline.draw_indexed(
                //     //     &mut pass,
                //     //     0..data.index_buffer().size() as u32,
                //     //     0..1,
                //     // );
                // });
                render_resource.instance_manager.prepare();
                for i in render_resource.instance_manager.iter_data() {
                    main_pipeline
                        .apply_groups(&mut pass, (i.texture_group(), camera_resource.group()));
                    main_pipeline.set_vertex_buffer(&mut pass, i.structure_buffer(), 0);
                    main_pipeline.set_buffer(&mut pass, i.transform_buffer(), 1);

                    main_pipeline.draw_indexed(
                        &mut pass,
                        0..i.structure_buffer().size() as u32,
                        0..i.instace_count(),
                    );
                }
            }
            render_resource.instance_manager.clear();

            render_resource
                .device()
                .queue()
                .submit(std::iter::once(encoder.finish()));
            output.present();
        } else if let Err(err) = current {
            match err {
                SurfaceError::Lost => {
                    render_resource.resize(&render_resource.window.inner_size());
                }
                _ => {
                    panic!("unrecoverable wgpu error!");
                }
            };
        };
        SystemControlFlow::Continue
    }
}

pub(super) type MainPipeline =
    MatrixRenderPipeline<(Vertex, InstanceTransform), ((MatrixTexture,), (CameraUniform,))>;
