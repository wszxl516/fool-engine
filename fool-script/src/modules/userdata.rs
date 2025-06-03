use crate::utils::set_module_name;
use mlua::{Function, Lua, Value};
use parking_lot::RwLock;
use std::{collections::HashMap, fmt::Debug, sync::Arc};
pub trait UserModConstructor: Send + Sync {
    fn call(&self, lua: &Lua) -> mlua::Result<Value>;
    fn clone_box(&self) -> Box<dyn UserModConstructor>;
}

impl<F> UserModConstructor for F
where
    F: Fn(&Lua) -> mlua::Result<Value> + Send + Sync + Clone + 'static,
{
    fn call(&self, lua: &Lua) -> mlua::Result<Value> {
        self(lua)
    }

    fn clone_box(&self) -> Box<dyn UserModConstructor> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn UserModConstructor> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
#[derive(Default, Clone)]
struct ModNode {
    children: HashMap<String, ModNode>,
    constructor: Option<Box<dyn UserModConstructor>>,
}

impl Debug for ModNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ModNode {{ children: {:?}, constructor: {}}}",
            self.children,
            if self.constructor.is_some() {
                "ModuleConstructor"
            } else {
                "None"
            }
        )
    }
}

#[repr(transparent)]
#[derive(Default, Debug, Clone)]
pub struct UserMod {
    root: Arc<RwLock<ModNode>>,
}

impl UserMod {
    pub fn new() -> Self {
        Self {
            root: Default::default(),
        }
    }

    pub fn register<F>(&self, path: &str, constructor: F)
    where
        F: UserModConstructor + 'static,
    {
        let parts = path.split('.').collect::<Vec<_>>();
        let mut node = &mut *self.root.write();

        for part in &parts[..parts.len() - 1] {
            node = node.children.entry(part.to_string()).or_default();
        }

        node.children
            .entry(parts.last().unwrap().to_string())
            .or_default()
            .constructor = Some(Box::new(constructor));
    }

    fn build_module_tree(lua: &Lua, node: &ModNode, parent_name: &str) -> mlua::Result<Value> {
        let table = lua.create_table()?;
        let mut current_path = parent_name.to_owned();
        for (k, child) in &node.children {
            current_path.push('.');
            current_path.push_str(k);
            let val = if let Some(constructor) = &child.constructor {
                constructor.call(lua)?
            } else if !child.children.is_empty() {
                Self::build_module_tree(lua, child, &current_path)?
            } else {
                continue;
            };

            let val = set_module_name(val, &current_path, lua)?;
            table.set(k.clone(), val)?;
        }

        Ok(Value::Table(table))
    }

    fn find_module(lua: &Lua, path: &str, node: &ModNode) -> mlua::Result<Value> {
        if path.is_empty() {
            return Err(mlua::Error::RuntimeError("module name is empty".into()));
        }

        let mut current = node;
        let parts = path.split('.').collect::<Vec<_>>();

        for part in &parts {
            current = current
                .children
                .get(*part)
                .ok_or_else(|| mlua::Error::RuntimeError(format!("module '{}' not found", path)))?;
        }

        let value = if let Some(constructor) = &current.constructor {
            constructor.call(lua)?
        } else {
            Self::build_module_tree(lua, current, path)?
        };
        let value = set_module_name(value, path, lua)?;
        Ok(value)
    }

    pub fn init(&self, lua: &Lua) -> mlua::Result<Function> {
        let root = self.root.clone();
        lua.create_function(move |lua, modname: String| {
            match Self::find_module(lua, &modname, &root.read()) {
                Ok(module) => {
                    let loader = lua.create_function(move |_, ()| Ok(module.clone()))?;
                    Ok((Value::Function(loader), lua.create_string(&modname)?))
                }
                Err(e) => {
                    log::error!("Module error: {}", e);
                    Ok((Value::Nil, lua.create_string("not found")?))
                }
            }
        })
    }
}
