use wgpu::{CommandEncoder, Device, Queue, SurfaceTexture, TextureView};

pub struct FrameContext {
    pub encoder: CommandEncoder,
    pub device: Device,
    pub queue: Queue,
    pub target_view: TextureView,
    pub surface_texture: SurfaceTexture,
}
