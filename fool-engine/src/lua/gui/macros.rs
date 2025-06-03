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
