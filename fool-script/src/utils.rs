use mlua::{Lua, Result, Value, Variadic};
use serde_json::Value as JsonValue;

pub fn lua_values_to_json_string(values: Variadic<Value>) -> Result<String> {
    fn convert(value: Value) -> Result<JsonValue> {
        Ok(match value {
            Value::Nil => JsonValue::Null,
            Value::Boolean(b) => JsonValue::Bool(b),
            Value::Integer(i) => JsonValue::Number(i.into()),
            Value::Number(n) => serde_json::Number::from_f64(n)
                .map(JsonValue::Number)
                .unwrap_or(JsonValue::Null),
            Value::String(s) => JsonValue::String(s.to_str()?.to_string()),
            Value::Table(t) => {
                let is_array = t
                    .clone()
                    .pairs::<Value, Value>()
                    .all(|r| matches!(r, Ok((Value::Integer(i), _)) if i >= 1));

                if is_array {
                    let mut vec = Vec::new();
                    for v in t.sequence_values::<Value>() {
                        vec.push(convert(v?)?);
                    }
                    JsonValue::Array(vec)
                } else {
                    let mut map = serde_json::Map::new();
                    for entry in t.pairs::<Value, Value>() {
                        let (k, v) = entry?;
                        let key = match k {
                            Value::String(s) => s.to_str()?.to_string(),
                            Value::Integer(i) => i.to_string(),
                            Value::Number(n) => n.to_string(),
                            _ => continue,
                        };
                        map.insert(key, convert(v)?);
                    }
                    JsonValue::Object(map)
                }
            }
            _ => JsonValue::String(format!("{:?}", value)),
        })
    }

    let mut json_array = String::new();
    for v in values {
        json_array.push_str(&serde_json::to_string_pretty(&convert(v)?).unwrap_or_default());
        json_array.push_str(", ");
    }
    Ok(json_array)
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
