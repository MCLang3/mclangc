use std::{default, str::FromStr, fmt::Display};
use clap::Parser;

use crate::VerboseLevel;

#[derive(Debug, Parser)]
pub struct Args {
    pub in_files: Vec<String>,
    /// The binary output file path
    #[arg(short='o', long="out")]
    pub out_file: String,
    /// Verbosity level, all possible values:
    /// 0: Quiet,
    /// 1: Normal,
    /// 2: Verbose,
    /// 3: Debug
    #[arg(short='v', long="verbose", default_value_t=crate::VerboseLevel::Normal)]
    pub verbosity: VerboseLevel
}


