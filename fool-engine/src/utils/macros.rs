#[macro_export]
macro_rules! lua_table_get {
    ($table: ident, $name: literal, $default: expr) => {
        $table.get($name).unwrap_or($default)
    };
}

#[cfg(feature = "debug")]
#[macro_export]
macro_rules! map2anyhow_error {
    ($code:expr, $msg:expr) => {{
        let loc = std::panic::Location::caller();
        $code.map_err(|e| {
            anyhow::anyhow!("{}, at {}:{} reason: {}", $msg, loc.file(), loc.line(), e)
        })
    }};
}

#[cfg(feature = "debug")]
#[macro_export]
macro_rules! map2lua_error {
    ($code:expr, $msg:expr) => {{
        let loc = std::panic::Location::caller();
        $code.map_err(|e| {
            mlua::Error::RuntimeError(format!(
                "{}, at {}:{} reason: {}",
                $msg,
                loc.file(),
                loc.line(),
                e
            ))
        })
    }};
}

#[cfg(not(feature = "debug"))]
#[macro_export]
macro_rules! map2anyhow_error {
    ($code:expr, $msg:expr) => {{
        $code.map_err(|e| anyhow::anyhow!("{}, reason: {}", $msg, e))
    }};
}

#[cfg(not(feature = "debug"))]
#[macro_export]
macro_rules! map2lua_error {
    ($code:expr, $msg:expr) => {{
        $code.map_err(|e| mlua::Error::RuntimeError(format!("{}, reason: {}", e, $msg)))
    }};
}

#[macro_export]
macro_rules! apply_if_some {
    ($target:ident,$method:ident, $field:expr) => {
        if let Some(val) = $field {
            $target = $target.$method(val);
        }
    };
    ($target:ident, $method:ident, $field:expr, $transform:expr) => {
        if let Some(val) = &$field {
            $target = $target.$method($transform(val));
        }
    };
}

#[macro_export]
macro_rules! lua_create_table {
    ($lua:ident, []) => {
        $lua.create_table()?
    };
    ($lua: ident, [$($field_name: ident = $field_value: expr), + $(,)?]) => {
        {
        let temp = $lua.create_table()?;
        $(temp.set(stringify!($field_name), $field_value)?;)*
        temp
        }
    };
}
