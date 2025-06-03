use egui::epaint::TextureHandle;
use image::DynamicImage;
use winit::{event_loop::ActiveEventLoop, window::CustomCursor};
pub fn create_cursor(
    event_loop: &ActiveEventLoop,
    img: &DynamicImage,
) -> anyhow::Result<CustomCursor> {
    let width = img.width() as u16;
    let height = img.height() as u16;
    let rgba = img.as_rgba8().cloned().unwrap().into_vec();
    let cursor = CustomCursor::from_rgba(rgba, width, height, width / 2, height / 2)?;
    Ok(event_loop.create_custom_cursor(cursor))
}

pub fn texture_from_image(
    name: &String,
    img: &image::DynamicImage,
    ctx: &egui::Context,
) -> anyhow::Result<TextureHandle> {
    use egui::ColorImage;
    use egui::TextureOptions;
    use image::GenericImageView;
    let rgba_image = img.to_rgba8();
    let (width, height) = (rgba_image.width() as usize, rgba_image.height() as usize);
    let pixels: Vec<egui::Color32> = img
        .pixels()
        .map(|p| egui::Color32::from_rgba_premultiplied(p.2 .0[0], p.2 .0[1], p.2 .0[2], p.2 .0[3]))
        .collect();
    let color_image = ColorImage {
        size: [width as usize, height as usize],
        pixels,
    };

    let ui_texture = ctx.load_texture(name, color_image, TextureOptions::default());
    Ok(ui_texture)
}
