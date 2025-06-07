use super::Fallback;
use dashmap::DashMap;
use parking_lot::RwLock;
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    hash::Hash,
    sync::Arc,
};
macro_rules! ResourceNotFound {
    ($name: expr) => {
        anyhow::anyhow!("Resource {} Not Found!", $name)
    };
}

pub trait ResId: Hash + Eq + Clone + Default + Display + Debug {}
pub trait ResData: Clone {}
impl<T> ResId for T where T: Hash + Eq + Clone + Default + Display + Debug {}
impl<T> ResData for T where T: Clone {}

#[derive(Clone, Debug)]
pub struct Resource<K, V>
where
    K: ResId,
    V: ResData,
{
    data: Arc<DashMap<K, V>>,
    fall_back: Arc<RwLock<Option<Box<dyn Fallback<K = K, V = V>>>>>,
}

impl<K: ResId, V: ResData> Default for Resource<K, V> {
    fn default() -> Resource<K, V> {
        Resource {
            data: Default::default(),
            fall_back: Default::default(),
        }
    }
}
impl<K, V> Resource<K, V>
where
    K: ResId,
    V: ResData,
{
    pub fn from_fallback(fall_back: impl Fallback<K = K, V = V> + 'static) -> Self {
        Self {
            data: Default::default(),
            fall_back: Arc::new(RwLock::new(Some(Box::new(fall_back)))),
        }
    }
    pub fn empty() -> Self {
        Self::default()
    }
    pub fn set_fall_back(&self, fall_back: impl Fallback<K = K, V = V> + 'static) {
        self.fall_back.write().replace(Box::new(fall_back));
    }
    pub fn load_from_map<KK: Into<K>, VV: Into<V>>(&self, map: HashMap<KK, VV>) {
        for (k, v) in map {
            let key = k.into();
            let data = v.into();
            self.data.insert(key, data);
        }
    }
    pub fn load(&self, name: impl Into<K>, data: impl Into<V>) {
        self.data.insert(name.into(), data.into());
    }
    pub fn get(&self, name: impl Into<K>) -> anyhow::Result<V> {
        let name = name.into();
        match self.data.get(&name) {
            Some(v) => Ok(v.value().clone()),
            None => match &self.fall_back.read().as_ref() {
                Some(fb) => match fb.get(&name) {
                    Ok(data) => {
                        let data = data;
                        log::trace!("load {} from Fallback {:?} succeed!!", &name, fb);
                        self.data.insert(name, data.clone());
                        Ok(data)
                    }
                    Err(err) => {
                        log::trace!("load {} from Fallback {:?} failed: {}!", &name, fb, err);
                        Err(ResourceNotFound!(name))
                    }
                },
                None => Err(ResourceNotFound!(name)),
            },
        }
    }
    pub fn remove(&self, path: impl Into<K>) {
        let path = path.into();
        self.data.remove(&path);
        log::trace!("remove resource: {}", &path);
    }
    pub fn exists(&self, name: impl Into<K>) -> bool {
        self.data.contains_key(&name.into())
    }
    pub fn list_names(&self) -> Vec<K> {
        self.data
            .iter()
            .map(|x| x.key().clone())
            .collect::<Vec<K>>()
    }
    pub fn count(&self) -> usize {
        self.data.len()
    }
}
