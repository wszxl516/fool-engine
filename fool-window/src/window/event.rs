use super::CustomEvent;
use image::DynamicImage;
use winit::{
    event_loop::{ActiveEventLoop, ControlFlow},
    window::{Cursor, CursorIcon, CustomCursor},
};
#[derive(Debug, Clone, Default)]
pub enum AppEvent {
    #[default]
    None,
    SetCursor(WindowCursor),
    ControlFlow(ControlFlow),
    Exit,
    CustomEvent(Box<dyn CustomEvent>),
}

#[derive(Debug, Clone, Default)]
pub enum WindowCursor {
    #[default]
    None,
    CursorIcon(CursorIcon),
    Image(DynamicImage),
}

impl WindowCursor {
    pub fn to_cursor(self, event_loop: &ActiveEventLoop) -> anyhow::Result<Option<Cursor>> {
        match self {
            Self::CursorIcon(icon) => Ok(Some(Cursor::Icon(icon))),
            Self::Image(img) => {
                let width = img.width() as u16;
                let height = img.height() as u16;
                let rgba = img
                    .as_rgba8()
                    .ok_or(anyhow::anyhow!(
                        "convert to rgba8 failed, wrong cursor format to_cursor"
                    ))?
                    .to_vec();
                let custom_cursor =
                    CustomCursor::from_rgba(rgba, width, height, width / 2, height / 2)?;
                let custom_cursor = event_loop.create_custom_cursor(custom_cursor);
                Ok(Some(Cursor::Custom(custom_cursor)))
            }
            Self::None => Ok(None),
        }
    }
}
