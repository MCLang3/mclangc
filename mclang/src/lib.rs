use std::{sync::RwLock, str::FromStr};

use lazy_static::lazy_static;

pub mod args;


pub struct GlobalData {
    pub verbosity: VerboseLevel
}

lazy_static!{
    pub static ref GLOBAL_DATA: RwLock<GlobalData> = {
        RwLock::new(GlobalData { 
            verbosity: VerboseLevel::Normal
        })
    };
}

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum VerboseLevel {
    Quiet = 0,
    #[default]
    Normal = 1,
    Verbose = 2,
    Debug = 3
}


impl std::fmt::Display for VerboseLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", *self as u8)
    }
}

impl FromStr for VerboseLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "0" | "q" | "quiet"   => Ok(Self::Quiet),
            "1" | "n" | "normal"  => Ok(Self::Normal),
            "2" | "v" | "verbose" => Ok(Self::Verbose),
            "3" | "d" | "debug"   => Ok(Self::Debug),
            s => {
                Err(format!("Invalid verbosity level: {}\nPossible values: 0: Quiet, 1: Normal, 2: Verbose, 3: Debug", s))
            }
        }
    }
}

