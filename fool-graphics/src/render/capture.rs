use std::path::PathBuf;
use wgpu::{
    Buffer, CommandEncoder, Device, Extent3d, SurfaceConfiguration, SurfaceTexture,
    TexelCopyBufferLayout,
};

#[derive(Debug)]
pub struct FrameCapture {
    buffer: Buffer,
    texture_extent: Extent3d,
    buffer_layout: TexelCopyBufferLayout,
    unpadded_bytes_per_row: u32,
    padded_bytes_per_row: u32,
    capture_to: PathBuf,
}
impl FrameCapture {
    pub fn new(config: &SurfaceConfiguration, device: &Device, capture_to: PathBuf) -> Self {
        let unpadded_bytes_per_row = config.width * 4;
        let padded_bytes_per_row = ((unpadded_bytes_per_row + 255) / 256) * 256;
        let buffer_size = (padded_bytes_per_row * config.height) as wgpu::BufferAddress;
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Readback Buffer"),
            size: buffer_size,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        let texture_extent = wgpu::Extent3d {
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1,
        };
        let buffer_layout = wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(padded_bytes_per_row),
            rows_per_image: Some(config.height),
        };
        Self {
            buffer,
            texture_extent,
            buffer_layout,
            unpadded_bytes_per_row,
            padded_bytes_per_row,
            capture_to,
        }
    }
    pub fn copy2buffer(
        &self,
        texture: &SurfaceTexture,
        encoder: &mut CommandEncoder,
    ) -> anyhow::Result<()> {
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture: &texture.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &self.buffer,
                layout: self.buffer_layout,
            },
            self.texture_extent,
        );
        Ok(())
    }
    pub fn finish(&self, device: &Device) {
        let buffer_slice = self.buffer.slice(..);
        buffer_slice.map_async(wgpu::MapMode::Read, |_| {});
        device.poll(wgpu::Maintain::Wait);
        let data = buffer_slice.get_mapped_range();
        let mut pixels = Vec::with_capacity(
            (self.texture_extent.width * self.texture_extent.height * 4) as usize,
        );
        for chunk in data.chunks(self.padded_bytes_per_row as usize) {
            let chunk = &chunk[..self.unpadded_bytes_per_row as usize];
            pixels.extend_from_slice(&chunk);
        }
        bgra_to_rgba(&mut pixels);
        let image_buffer = image::RgbaImage::from_raw(
            self.texture_extent.width,
            self.texture_extent.height,
            pixels,
        );
        if let Some(img_buf) = image_buffer {
            if let Err(err) = img_buf.save_with_format(&self.capture_to, image::ImageFormat::Png) {
                log::error!("image capture {} failed {}", self.capture_to.display(), err)
            } else {
                log::debug!("image capture {} finished!", self.capture_to.display())
            }
        } else {
            log::error!("image capture {} failed!", self.capture_to.display())
        }
        drop(data);
        self.buffer.unmap();
    }
}

#[inline]
fn bgra_to_rgba(pixels: &mut [u8]) {
    for chunk in pixels.chunks_exact_mut(4) {
        chunk.swap(0, 2);
    }
}
