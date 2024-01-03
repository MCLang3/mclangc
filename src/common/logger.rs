use crate::args::VerboseLevel;

use super::Loc;

#[allow(dead_code)] const RESET: &str = "\x1B[0m";
#[allow(dead_code)] const RED: &str = "\x1B[31m";
#[allow(dead_code)] const YELLOW: &str = "\x1B[33m";
#[allow(dead_code)] const BLUE: &str = "\x1B[34m";
#[allow(dead_code)] const GREEN: &str = "\x1B[32m";
#[allow(dead_code)] const BOLD: &str = "\x1B[1m";

pub fn _lerror(loc: &Loc, s: String) {
    eprintln!("{loc}:{RED}{BOLD}error{RESET}: {text}",
        loc=loc.human(),
        text=s,
    );
}
pub fn _lwarn(loc: &Loc, s: String) {
    if crate::GLOBAL_DATA.read().unwrap().verbosity < VerboseLevel::Normal {
        return;
    }
    println!("{loc}:{YELLOW}{BOLD}warn{RESET}: {text}",
        loc=loc.human(),
        text=s,
    );
}
pub fn _linfo(loc: &Loc, s: String) {
    if crate::GLOBAL_DATA.read().unwrap().verbosity < VerboseLevel::Normal {
        return;
    }
    println!("{loc}:{GREEN}{BOLD}info{RESET}: {text}",
        loc=loc.human(),
        text=s,
    );
}
pub fn _ldebug(loc: &Loc, s: String) {
    if crate::GLOBAL_DATA.read().unwrap().verbosity > VerboseLevel::Debug {
        return;
    }
    eprintln!("{loc}:{BLUE}{BOLD}debug{RESET}: {text}",
        loc=loc.human(),
        text=s,
    );
}


pub fn _error(s: String) {
    eprintln!("{RED}{BOLD}error{RESET}: {text}",
        text=s,
    );
}
pub fn _warn(s: String) {
    if crate::GLOBAL_DATA.read().unwrap().verbosity < VerboseLevel::Normal {
        return;
    }
    println!("{YELLOW}{BOLD}warn{RESET}: {text}",
        text=s,
    );
}
pub fn _info(s: String) {
    if crate::GLOBAL_DATA.read().unwrap().verbosity < VerboseLevel::Normal {
        return;
    }
    println!("{GREEN}{BOLD}info{RESET}: {text}",
        text=s,
    );
}
pub fn _debug(s: String) {
    if crate::GLOBAL_DATA.read().unwrap().verbosity > VerboseLevel::Debug {
        return;
    }
    eprintln!("{BLUE}{BOLD}debug{RESET}: {text}",
        text=s,
    );
}

#[macro_export]
macro_rules! error {
    ($loc:expr, $s:literal, $($args:tt)*) => {
        $crate::common::logger::_lerror($loc, format!($s, $($args)*))
    };
    ($s:literal, $($args:tt)*) => {
        $crate::common::logger::_error(format!($s, $($args)*))
    };
    ($s:literal) => {
        $crate::common::logger::_error($s.to_string())
    };
}
#[macro_export]
macro_rules! warn {
    ($loc:expr, $s:literal, $($args:tt)*) => {
        $crate::common::logger::_lwarn($loc, format!($s, $($args)*))
    };
    ($($args:tt)*) => {
        $crate::common::logger::_warn(format!($($args)*))
    };
    ($s:literal) => {
        $crate::common::logger::_warn($s.to_string())
    };
}
#[macro_export]
macro_rules! info {
    ($loc:expr, $s:literal, $($args:tt)*) => {
        $crate::common::logger::_linfo($loc, format!($s, $($args)*))
    };
    ($($args:tt)*) => {
        $crate::common::logger::_info(format!($($args)*))
    };
    ($s:literal) => {
        $crate::common::logger::_info($s.to_string())
    };
}
#[macro_export]
macro_rules! debug {
    ($loc:expr, $s:literal, $($args:tt)*) => {
        $crate::common::logger::_ldebug($loc, format!($s, $($args)*))
    };
    ($($args:tt)*) => {
        $crate::common::logger::_debug(format!($($args)*))
    };
    ($s:literal) => {
        $crate::common::logger::_debug($s.to_string())
    };
}
