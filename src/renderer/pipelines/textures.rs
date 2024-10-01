use image::ImageError;
use wgpu::{Extent3d, Texture};

use super::device_queue::DeviceQueue;

pub struct MatrixTexture {
    texture: Texture,
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
        Ok(Self { texture })
    }
}
