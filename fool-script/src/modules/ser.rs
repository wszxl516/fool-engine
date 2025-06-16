#![allow(dead_code)]
use bson::{Bson, Document};
use mlua::{Lua, Table, Value};

pub fn lua_value_to_bson(value: Value) -> mlua::Result<Bson> {
    Ok(match value {
        Value::Nil => Bson::Null,
        Value::Boolean(b) => Bson::Boolean(b),
        Value::Integer(i) => Bson::Int64(i),
        Value::Number(f) => Bson::Double(f),
        Value::String(s) => Bson::String(s.to_str()?.to_owned()),
        Value::Table(t) => {
            if is_array_table(&t)? {
                let mut arr = Vec::new();
                for pair in t.sequence_values::<Value>() {
                    let v = lua_value_to_bson(pair?)?;
                    arr.push(v);
                }
                Bson::Array(arr)
            } else {
                let doc = lua_table_to_bson_doc(&t)?;
                Bson::Document(doc)
            }
        }
        _ => return Err(mlua::Error::RuntimeError("Unsupported Lua value".into())),
    })
}

fn lua_table_to_bson_doc(table: &Table) -> mlua::Result<Document> {
    let mut doc = Document::new();
    for pair in table.pairs::<Value, Value>() {
        let (k, v) = pair?;
        let key = match k {
            Value::String(s) => s.to_str()?.to_owned(),
            Value::Integer(i) => i.to_string(),
            _ => return Err(mlua::Error::RuntimeError("Unsupported key type".into())),
        };
        doc.insert(key, lua_value_to_bson(v)?);
    }
    Ok(doc)
}

fn is_array_table(table: &Table) -> mlua::Result<bool> {
    let len = table.raw_len();
    for pair in table.clone().pairs::<Value, Value>() {
        let (key, _) = pair?;
        match key {
            Value::Integer(i) if i >= 1 && i <= len as i64 => continue,
            _ => return Ok(false),
        }
    }
    Ok(true)
}

pub fn bson_to_lua_value(lua: &Lua, value: &Bson) -> mlua::Result<Value> {
    Ok(match value {
        Bson::Null => Value::Nil,
        Bson::Boolean(b) => Value::Boolean(*b),
        Bson::Int32(i) => Value::Integer(*i as i64),
        Bson::Int64(i) => Value::Integer(*i),
        Bson::Double(f) => Value::Number(*f),
        Bson::String(s) => Value::String(lua.create_string(&s)?),
        Bson::Array(arr) => {
            let table = lua.create_table()?;
            for (i, item) in arr.into_iter().enumerate() {
                table.set((i + 1) as i64, bson_to_lua_value(lua, item)?)?;
            }
            Value::Table(table)
        }
        Bson::Document(doc) => Value::Table(bson_doc_to_lua_table(lua, &doc)?),
        _ => Value::Nil,
    })
}

fn bson_doc_to_lua_table<'lua>(lua: &'lua Lua, doc: &Document) -> mlua::Result<Table> {
    let table = lua.create_table()?;
    for (k, v) in doc.iter() {
        table.set(k.as_str(), bson_to_lua_value(lua, v)?)?;
    }
    Ok(table)
}
