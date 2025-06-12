use crate::{map2lua_error, script::types::LuaPoint};
pub use fool_graphics::canvas::{Animation, Frame, SceneGraph, Sprite};
use mlua::UserData;
use parking_lot::RwLock;
use std::sync::Arc;
pub struct LuaSrpite {
    pub scene_graph: Arc<RwLock<SceneGraph>>,
    pub sprite: Sprite<usize>,
}

impl UserData for LuaSrpite {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut(
            "create_animation",
            |_lua, this, (name, frames_id, fps): (String, Vec<usize>, u32)| {
                let animation = map2lua_error!(
                    this.sprite
                        .create_animation(name, frames_id.into_iter(), fps),
                    "create_animation"
                )?;
                Ok(LuaAnimation {
                    animation,
                    scene_graph: this.scene_graph.clone(),
                })
            },
        );
        methods.add_method_mut("get_animation", |_lua, this, name: String| {
            let animation = map2lua_error!(this.sprite.get_animation(name), "get_animation")?;
            Ok(LuaAnimation {
                animation,
                scene_graph: this.scene_graph.clone(),
            })
        });
        methods.add_method_mut("list_animation", |_lua, this, ()| {
            let animations = this.sprite.list_animation();
            Ok(animations)
        });
    }
}

pub struct LuaAnimation {
    animation: Animation,
    scene_graph: Arc<RwLock<SceneGraph>>,
}

impl UserData for LuaAnimation {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("count", |_lua, this, ()| Ok(this.animation.count()));
        methods.add_method_mut("current", |_lua, this, ()| Ok(this.animation.current()));
        methods.add_method_mut("next", |_lua, this, ()| Ok(this.animation.next()));
        methods.add_method_mut("draw", |_lua, this, pos: LuaPoint<f64>| {
            let node = this.animation.to_node(pos.x, pos.y);
            this.scene_graph.write().root.add_child(&node);
            Ok(())
        });
    }
}
