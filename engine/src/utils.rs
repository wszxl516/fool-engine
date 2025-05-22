#[macro_export]
macro_rules! map2anyhow_error {
    ($code: expr, $message:expr) => {
        $code.map_err(|e| anyhow::anyhow!("{}: {}", $message, e))?
    };
}
#[macro_export]
macro_rules! map2lua_error {
    ($code: expr, $message:expr) => {
        $code.map_err(|e| mlua::Error::RuntimeError(format!("{}, {}", $message, e)))?
    };
}

#[macro_export]
macro_rules! lua_table_get {
    ($table: ident, $name: literal, $default: expr) => {
        $table.get($name).unwrap_or($default)
    };
}
