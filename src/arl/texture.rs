use image::{DynamicImage, GenericImageView};

use crate::arl::{
    bind_groups::{BindGroupLayoutEntry, BindGroupable},
    device_queue::DeviceQueue,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextureID {
    pub path: &'static str,
}

pub struct TextureArgs<'a> {
    label: String,
    dynamic_image: &'a DynamicImage,
    path: &'static str,
    usage: wgpu::TextureUsages,
}

pub struct Texture {
    id: TextureID,
    label: String,
    texture: wgpu::Texture,
    sampler: wgpu::Sampler,
    view: wgpu::TextureView,
}

impl Texture {
    pub fn from_memory(args: TextureArgs<'_>, device_queue: &DeviceQueue) -> Self {
        let dim = args.dynamic_image.dimensions();
        let texture_size = wgpu::Extent3d {
            width: dim.0,
            height: dim.1,
            depth_or_array_layers: 1,
        };
        let diffuse_rgba = args.dynamic_image.to_rgba8();

        let texture = device_queue
            .device()
            .create_texture(&wgpu::wgt::TextureDescriptor {
                label: Some(&args.label),
                size: texture_size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: args.usage,
                view_formats: &[],
            });

        device_queue.queue().write_texture(
            wgpu::TexelCopyTextureInfoBase {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &diffuse_rgba,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * dim.0),
                rows_per_image: Some(dim.1),
            },
            texture_size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device_queue
            .device()
            .create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            });

        Self {
            id: TextureID { path: args.path },
            label: args.label,
            sampler,
            texture,
            view,
        }
    }

    pub fn raw(&self) -> &wgpu::Texture {
        &self.texture
    }

    pub fn sampler(&self) -> &wgpu::Sampler {
        &self.sampler
    }

    pub fn view(&self) -> &wgpu::TextureView {
        &self.view
    }
}

impl BindGroupable for Texture {
    type BindGroupID = TextureID;

    fn new(id: &Self::BindGroupID, device_queue: &DeviceQueue) -> Self {
        let bytes = std::fs::read(id.path).unwrap();
        let dynamic_image = image::load_from_memory(&bytes).unwrap();
        Texture::from_memory(
            TextureArgs {
                path: id.path,
                dynamic_image: &dynamic_image,
                label: format!("bind group texture '{}'", id.path),
                usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
            },
            device_queue,
        )
    }

    fn label(&self) -> String {
        self.label.clone()
    }

    fn layout_label() -> String {
        "bind group texture layout".to_string()
    }

    fn get_layout_entries() -> &'static [super::bind_groups::BindGroupLayoutEntry] {
        &[
            BindGroupLayoutEntry {
                count: None,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                visibility: wgpu::ShaderStages::FRAGMENT,
            },
            BindGroupLayoutEntry {
                count: None,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                visibility: wgpu::ShaderStages::FRAGMENT,
            },
        ]
    }

    fn get_group_entries(&self) -> impl AsRef<[wgpu::BindingResource<'_>]> {
        [
            wgpu::BindingResource::TextureView(&self.view),
            wgpu::BindingResource::Sampler(&self.sampler),
        ]
    }

    fn id(&self) -> Self::BindGroupID {
        Self::BindGroupID { path: self.id.path }
    }
}
