
#[macro_export]
macro_rules! error {
    ($loc:expr, $s:literal, $($args:tt)*) => {
            $crate::logger::_lerror($loc, format!($s, $($args)*))
    };
    ($s:literal, $($args:tt)*) => {
        $crate::logger::_error(format!($s, $($args)*))
    };
    ($s:literal) => {
        $crate::logger::_error($s.to_string())
    };
}

#[macro_export]
macro_rules! warn {
    ($loc:expr, $s:literal, $($args:tt)*) => {
        $crate::logger::_lwarn($loc, format!($s, $($args)*))
    };
    ($($args:tt)*) => {
        $crate::logger::_warn(format!($($args)*))
    };
    ($s:literal) => {
        $crate::logger::_warn($s.to_string())
    };
}

#[macro_export]
macro_rules! info {
    ($loc:expr, $s:literal, $($args:tt)*) => {
        $crate::logger::_linfo($loc, format!($s, $($args)*))
    };
    ($($args:tt)*) => {
        $crate::logger::_info(format!($($args)*))
    };
    ($s:literal) => {
        $crate::logger::_info($s.to_string())
    };
}

#[macro_export]
macro_rules! debug {
    ($loc:expr, $s:literal, $($args:tt)*) => {
        $crate::logger::_ldebug($loc, format!($s, $($args)*))
    };
    ($($args:tt)*) => {
        $crate::logger::_debug(format!($($args)*))
    };
    ($s:literal) => {
        $crate::logger::_debug($s.to_string())
    };
}