use crate::{
    apply_if_some,
    script::types::{LuaPoint, LuaSize},
};
use fool_window::CustomEvent;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use winit::{
    dpi::{LogicalPosition, LogicalSize, Position, Size},
    event_loop::EventLoop,
    window::{
        Cursor, CursorIcon, CustomCursor, Icon, Theme, WindowAttributes, WindowButtons, WindowLevel,
    },
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum WinLevel {
    AlwaysOnBottom,
    #[default]
    Normal,
    AlwaysOnTop,
}

impl Into<WindowLevel> for WinLevel {
    fn into(self) -> WindowLevel {
        match self {
            WinLevel::AlwaysOnBottom => WindowLevel::AlwaysOnBottom,
            WinLevel::AlwaysOnTop => WindowLevel::AlwaysOnTop,
            WinLevel::Normal => WindowLevel::Normal,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WinButtons(pub Vec<String>);
impl Default for WinButtons {
    fn default() -> Self {
        Self(vec![
            "close".to_owned(),
            "maximize".to_owned(),
            "minimize".to_owned(),
        ])
    }
}
impl Into<WindowButtons> for WinButtons {
    fn into(self) -> WindowButtons {
        if self.0.is_empty() {
            return WindowButtons::all();
        }
        let mut all = WindowButtons::empty();
        for item in self.0 {
            match item.to_ascii_lowercase().as_str() {
                "close" => all |= WindowButtons::CLOSE,
                "maximize" => all |= WindowButtons::MAXIMIZE,
                "minimize" => all |= WindowButtons::MINIMIZE,
                _ => {}
            }
        }
        all
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    pub defailt_size: LuaSize<f64>,
    pub min_size: Option<LuaSize<f64>>,
    pub max_size: Option<LuaSize<f64>>,
    pub position: Option<LuaPoint<f64>>,
    pub resizable: Option<bool>,
    pub enabled_buttons: Option<WinButtons>,
    pub title: Option<String>,
    pub maximized: Option<bool>,
    pub visible: Option<bool>,
    pub transparent: Option<bool>,
    pub blur: Option<bool>,
    pub decorations: Option<bool>,
    pub window_icon: Option<String>,
    pub preferred_theme: Option<Theme>,
    pub resize_increments: Option<LuaSize<f64>>,
    pub content_protected: Option<bool>,
    pub window_level: Option<WinLevel>,
    pub active: Option<bool>,
    pub cursor: Option<String>,
    pub fullscreen: Option<bool>,
}

impl WindowConfig {
    pub fn build(
        &self,
        event_loop: &EventLoop<impl CustomEvent>,
    ) -> anyhow::Result<WindowAttributes> {
        let mut attributes = WindowAttributes::default()
            .with_active(self.active.unwrap_or(true))
            .with_window_level(
                self.window_level
                    .clone()
                    .map(|l| l.into())
                    .unwrap_or_default(),
            )
            .with_content_protected(self.content_protected.unwrap_or(false))
            .with_theme(self.preferred_theme)
            .with_decorations(self.decorations.unwrap_or(true))
            .with_blur(self.blur.unwrap_or(false))
            .with_transparent(self.transparent.unwrap_or(false))
            .with_visible(self.visible.unwrap_or(true))
            .with_maximized(self.maximized.unwrap_or(true))
            .with_title(self.title.clone().unwrap_or("Fool Engine".to_owned()))
            .with_enabled_buttons(self.enabled_buttons.clone().unwrap_or_default().into())
            .with_resizable(self.resizable.unwrap_or(true))
            .with_inner_size(Size::Logical(LogicalSize::new(
                self.defailt_size.width,
                self.defailt_size.height,
            )));
        apply_if_some!(
            attributes,
            with_position,
            self.position.clone(),
            |pos: &LuaPoint<f64>| { Position::Logical(LogicalPosition::new(pos.x, pos.y)) }
        );
        apply_if_some!(
            attributes,
            with_resize_increments,
            self.resize_increments,
            |size: &LuaSize<f64>| { Size::Logical(LogicalSize::new(size.width, size.height)) }
        );
        apply_if_some!(
            attributes,
            with_min_inner_size,
            self.min_size,
            |size: &LuaSize<f64>| { Size::Logical(LogicalSize::new(size.width, size.height)) }
        );
        apply_if_some!(
            attributes,
            with_max_inner_size,
            self.max_size,
            |size: &LuaSize<f64>| { Size::Logical(LogicalSize::new(size.width, size.height)) }
        );
        if let Some(cursor) = &self.cursor {
            let cursor = create_cursor(event_loop, &cursor)?;
            attributes = attributes.with_cursor(cursor);
        }
        if let Some(icon) = &self.window_icon {
            let cursor = create_icon(&icon)?;
            attributes = attributes.with_window_icon(Some(cursor));
        }
        if Some(true) == self.fullscreen {
            attributes =
                attributes.with_fullscreen(Some(winit::window::Fullscreen::Borderless(None)))
        }
        Ok(attributes)
    }
}

pub fn create_cursor(
    event_loop: &EventLoop<impl CustomEvent>,
    img_path: &String,
) -> anyhow::Result<Cursor> {
    if let Ok(cur) = CursorIcon::from_str(&img_path) {
        Ok(Cursor::Icon(cur))
    } else {
        let buffer = super::utils::load_from_current(&img_path)?;
        let img = image::load_from_memory(&buffer)?;
        let width = img.width() as u16;
        let height = img.height() as u16;
        let rgba = img
            .as_rgba8()
            .ok_or(anyhow::anyhow!("{}, wrong cursor format", img_path))?
            .to_vec();
        let cursor = CustomCursor::from_rgba(rgba, width, height, width / 2, height / 2)?;
        Ok(Cursor::Custom(event_loop.create_custom_cursor(cursor)))
    }
}

pub fn create_icon(img_name: &String) -> anyhow::Result<Icon> {
    let buffer = super::utils::load_from_current(&img_name)?;
    let img = image::load_from_memory(&buffer)?;
    let width = img.width();
    let height = img.height();
    let rgba = img
        .as_rgba8()
        .ok_or(anyhow::anyhow!("convert {} to rgba8 failed!", img_name))?;
    Ok(Icon::from_rgba(rgba.clone().into_vec(), width, height)?)
}
