use mlua::{Lua, Result, Value, Variadic};
fn value_convert(value: &Value, depth: usize) -> String {
    let indent = "  ".repeat(depth);
    match value {
        Value::Nil => "nil".into(),
        Value::Boolean(b) => b.to_string(),
        Value::Integer(i) => i.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => s.to_string_lossy().to_string(),
        Value::Table(t) => {
            let mut out = String::new();
            out.push_str("{\n");
            for pair in t.pairs::<Value, Value>() {
                match pair {
                    Ok((k, v)) => {
                        let key_str = value_convert(&k, depth + 1);
                        let val_str = value_convert(&v, depth + 1);
                        out.push_str(&format!("{}  {} = {},\n", indent, key_str, val_str));
                    }
                    Err(_) => {}
                }
            }
            out.push_str(&format!("{}}}", indent));
            out
        }
        Value::Function(f) => {
            format!("<Function {}>", f.info().name.unwrap_or("<anymous>".into()))
        }
        Value::UserData(_) => "<UserData>".into(),
        Value::LightUserData(_) => "<lightuserdata>".into(),
        Value::Thread(_) => "<thread>".into(),
        Value::Error(e) => format!("<error: {}>", e),
        Value::Other(o) => format!("<other: {:?}>", o),
    }
}
pub fn values_to_string(values: &Variadic<Value>) -> Result<Vec<String>> {
    let mut buffer = Vec::new();
    for val in values.iter() {
        buffer.push(value_convert(val, 0));
    }
    Ok(buffer)
}

pub fn dump_lua_stack_trace(lua: &Lua) {
    log::error!("--- Lua Stack Trace ---");
    let mut level = 0;
    while let Some(debug) = lua.inspect_stack(level) {
        let name = debug.names().name.unwrap_or("<anonymous>".into());
        let source = debug.source().source.unwrap_or("<unknown>".into());
        let line = debug.curr_line();
        let what = debug.event();
        log::error!("#{:<2} [{:?}] {}:{}:{}", level, what, source, name, line);
        level += 1;
    }
    log::error!("--- Lua Stack Trace ---");
}

pub fn set_module_name(value: Value, path: &str, lua: &Lua) -> mlua::Result<Value> {
    let value = match &value {
        Value::Table(t) => {
            t.set("__modname", path)?;
            Value::Table(t.clone())
        }
        Value::UserData(u) => {
            let wrapper = lua.create_table()?;
            let mt = lua.create_table()?;
            mt.set("__index", u.clone())?;
            wrapper.set_metatable(Some(mt));
            wrapper.set("__modname", path)?;
            Value::Table(wrapper)
        }
        Value::LightUserData(u) => {
            let wrapper = lua.create_table()?;
            let mt = lua.create_table()?;
            mt.set("__index", u.clone())?;
            wrapper.set_metatable(Some(mt));
            wrapper.set("__modname", path)?;
            Value::Table(wrapper)
        }
        _ => value,
    };
    Ok(value)
}
