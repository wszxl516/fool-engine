#[macro_export]
macro_rules! lua_table_get {
    ($table: ident, $name: literal, $default: expr) => {
        $table.get($name).unwrap_or($default)
    };
}

#[macro_export]
macro_rules! log_error_exit {
    ($($arg:tt)+) => ({
        log::error!($($arg)+);
        std::process::exit(-1);
    });
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
    ($code:expr, $msg:expr) => {{ $code.map_err(|e| anyhow::anyhow!("{}, reason: {}", $msg, e)) }};
}

#[cfg(not(feature = "debug"))]
#[macro_export]
macro_rules! map2lua_error {
    ($code:expr, $msg:expr) => {{ $code.map_err(|e| mlua::Error::RuntimeError(format!("{}, reason: {}", e, $msg))) }};
}
