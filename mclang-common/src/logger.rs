use mclang::VerboseLevel;
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
    if mclang::GLOBAL_DATA.read().unwrap().verbosity < VerboseLevel::Normal {
        return;
    }
    println!("{loc}:{YELLOW}{BOLD}warn{RESET}: {text}",
        loc=loc.human(),
        text=s,
    );
}
pub fn _linfo(loc: &Loc, s: String) {
    if mclang::GLOBAL_DATA.read().unwrap().verbosity < VerboseLevel::Normal {
        return;
    }
    println!("{loc}:{GREEN}{BOLD}info{RESET}: {text}",
        loc=loc.human(),
        text=s,
    );
}
pub fn _ldebug(loc: &Loc, s: String) {
    if mclang::GLOBAL_DATA.read().unwrap().verbosity > VerboseLevel::Debug {
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
    if mclang::GLOBAL_DATA.read().unwrap().verbosity < VerboseLevel::Normal {
        return;
    }
    println!("{YELLOW}{BOLD}warn{RESET}: {text}",
        text=s,
    );
}
pub fn _info(s: String) {
    if mclang::GLOBAL_DATA.read().unwrap().verbosity < VerboseLevel::Normal {
        return;
    }
    println!("{GREEN}{BOLD}info{RESET}: {text}",
        text=s,
    );
}
pub fn _debug(s: String) {
    if mclang::GLOBAL_DATA.read().unwrap().verbosity > VerboseLevel::Debug {
        return;
    }
    eprintln!("{BLUE}{BOLD}debug{RESET}: {text}",
        text=s,
    );
}
