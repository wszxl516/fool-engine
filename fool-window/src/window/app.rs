use super::input::WinEvent;
use std::{fmt::Debug, sync::Arc};
pub use winit::{event::WindowEvent, window::Window};

use super::EventProxy;
use dyn_clone::DynClone;

dyn_clone::clone_trait_object!(CustomEvent);

pub trait CustomEvent: DynClone + Send + Sync + Debug + 'static {}
impl<T: DynClone + Send + Sync + Debug + 'static> CustomEvent for T {}
pub trait Application: Send {
    fn init(&mut self, window: Arc<Window>, proxy: &EventProxy);
    fn update(&mut self);
    fn event(&mut self, event: &WinEvent, raw_event: &WindowEvent);
    fn exiting(&mut self);
    fn user_event(&mut self, _event: Box<dyn CustomEvent>) {}
}
