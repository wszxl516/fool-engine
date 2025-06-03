use super::resource::{ResData, ResId};
use dyn_clone::DynClone;
use std::fmt::Debug;
pub trait Fallback: Send + Sync + DynClone + Debug {
    type K: ResId;
    type V: ResData;

    fn get(&self, key: &Self::K) -> anyhow::Result<Self::V>;
}

dyn_clone::clone_trait_object!(<K: ResId, V: ResData> Fallback<K = K, V = V>);
