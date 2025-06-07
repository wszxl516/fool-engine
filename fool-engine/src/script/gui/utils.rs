use egui::Color32;
pub fn adjust_brightness(color: Color32, factor: f32) -> Color32 {
    let r = (color.r() as f32 * factor).clamp(80.0, 255.0) as u8;
    let g = (color.g() as f32 * factor).clamp(80.0, 255.0) as u8;
    let b = (color.b() as f32 * factor).clamp(80.0, 255.0) as u8;
    let a = (color.a() as f32 * factor).clamp(80.0, 255.0) as u8;
    Color32::from_rgba_unmultiplied(r, g, b, a)
}
