/// Struct pairing a set of values and the generic defaults returning the
/// generic one when the actual value is not set.
pub struct ActualAndDefaultCfg<T> {
    pub actual: Box<T>,
    pub default: Box<T>,
}

/// Return the actual value, the generic value or the default value for the
/// field. The struct must have `actual` and `default` fields of the same type.
#[macro_export]
macro_rules! actual_or_default {
    ($struct: ident, $field: ident) => {
        if let Some(ref value) = $struct.actual.$field {
            value.clone()
        } else {
            $struct.default.$field.clone().unwrap_or_default()
        }
    };
}
