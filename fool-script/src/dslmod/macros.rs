#[macro_export]
macro_rules! impl_sringnify_enum {
    {
    $(#[$inner:meta])* $vis:vis $name:ident [$($field_name: tt), + $(,)?]
    }  => {
        $(#[$inner])*
        $vis enum $name{
            $($field_name,)*
        }
        impl $name{
            pub fn from_value(bits: &str) -> Option<Self> {
                match bits {
                    $(stringify!($field_name) => Some(Self::$field_name),)*
                    _ => None
                }
            }
            pub fn into_value(self) -> &'static str {
                match self {
                    $(Self::$field_name => stringify!(Self::$field_name),)*
                }
            }
        }
    };
}
