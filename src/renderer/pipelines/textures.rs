use std::{fs, path::Path};

use image::ImageError;
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, Extent3d, Sampler, ShaderStages, Texture, TextureView,
};

use super::{
    bind_groups::{bind::MatrixBindable, bind_group::MatrixBindGroupable},
    device_queue::DeviceQueue,
};

pub struct MatrixTexture {
    texture: Texture,
    view: TextureView,
    sampler: Sampler,
}

impl MatrixTexture {
    pub fn new(device_queue: &DeviceQueue, image_raw: &[u8]) -> Result<Self, ImageError> {
        let diff = image::load_from_memory(image_raw)?;

        let diff_rgba = diff.to_rgba8();

        let dimensions = diff_rgba.dimensions();

        let texture_size = Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let texture = device_queue
            .device()
            .create_texture(&wgpu::TextureDescriptor {
                size: texture_size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                label: Some("diffuse_texture"),
                view_formats: &[],
            });
        device_queue.queue().write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &diff_rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
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
        Ok(Self {
            texture,
            view,
            sampler,
        })
    }

    pub(crate) fn from_path(
        device_queue: &DeviceQueue,
        texture_path: impl AsRef<Path>,
    ) -> Result<MatrixTexture, ImageError> {
        Self::new(
            device_queue,
            &fs::read(texture_path).expect("this file should exist"),
        )
    }
}

impl MatrixBindGroupable for MatrixTexture {
    fn create_group_layout(device_queue: &DeviceQueue) -> wgpu::BindGroupLayout {
        device_queue
            .device()
            .create_bind_group_layout(&BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            })
    }

    fn create_group(
        &self,
        device_queue: &DeviceQueue,
        layout: &super::bind_groups::bind_group::MatrixBindGroupLayout<Self>,
    ) -> BindGroup
    where
        Self: Sized,
    {
        device_queue
            .device()
            .create_bind_group(&BindGroupDescriptor {
                layout: layout.layout(),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&self.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&self.sampler),
                    },
                ],
                label: Some("diffuse_bind_group"),
            })
    }
}
