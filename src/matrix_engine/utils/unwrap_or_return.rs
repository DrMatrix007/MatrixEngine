#[macro_export]
macro_rules! unwrap_or_return {
    ($_val:expr,$default_value:expr) => {
        {
            match $_val {
                Some(val) => val,
                None => return $default_value,
            }
        }
    };
    ($_val:expr) => {
        unwrap_or_return!($_val,())
    };
}