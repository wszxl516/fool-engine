use super::EventProxy;
use super::input::WinEvent;
use downcast_rs::{Downcast, impl_downcast};
use dyn_clone::{DynClone, clone_trait_object};
use std::{fmt::Debug, sync::Arc};
pub use winit::{event::WindowEvent, window::Window};

pub trait CustomEvent: Downcast + DynClone + Send + Sync + Debug + 'static {}
clone_trait_object!(CustomEvent);
impl_downcast!(CustomEvent);
impl<T: Downcast + DynClone + Send + Sync + Debug + 'static> CustomEvent for T {}
pub trait Application {
    fn init(&mut self, window: Arc<Window>, proxy: &EventProxy);
    fn update(&mut self);
    fn event(&mut self, event: &WinEvent, raw_event: &WindowEvent);
    fn exiting(&mut self);
    fn user_event(&mut self, _event: Box<dyn CustomEvent>) {}
}
