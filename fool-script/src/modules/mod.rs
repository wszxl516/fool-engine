#![allow(dead_code)]
#![allow(unused_imports)]
mod dsl;
mod memory;
pub mod ser;
pub mod stdlib;
mod userdata;
pub use dsl::{DSLContent, DSLID, DSLModule, ModKind};
use fool_resource::{Resource, SharedData};
pub use memory::MemoryModule;
pub use userdata::{UserMod, UserModConstructor};

#[derive(Debug, Clone, Default)]
pub struct Modules {
    pub mem_mod: MemoryModule,
    pub dsl_mod: DSLModule,
    pub user_mod: UserMod,
}
