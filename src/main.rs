#[macro_use]
pub(crate) mod common;
pub(crate) mod util;
pub(crate) mod parser;
pub(crate) mod tokeniser;
mod args;


use std::sync::RwLock;

use anyhow::Result;
use clap::Parser;
use lazy_static::lazy_static;

use crate::args::VerboseLevel;

struct GlobalData {
    verbosity: args::VerboseLevel
}

lazy_static!{
    static ref GLOBAL_DATA: RwLock<GlobalData> = {
        RwLock::new(GlobalData { 
            verbosity: args::VerboseLevel::Normal
        })
    };
}

fn main() -> Result<()> {
    std::process::exit(main2()?);
}

fn main2() -> Result<i32> {


    let args = args::Args::parse();

    GLOBAL_DATA.write().unwrap().verbosity = args.verbosity;

    let filed = std::fs::read_to_string(args.in_files[0].clone())?;

    let tokens = tokeniser::Tokeniser::new(&filed, args.in_files[0].clone())
        .parse()
        .tokens();
    // let mut ln = 1;
    // for token in tokens {
    //     if token.loc.line > ln {
    //         print!("\n{} ", token.to_string());
    //         ln = token.loc.line;
    //     } else {
    //         print!("{} ", token.to_string());
    //     }
    // }
 
    let mut parser = parser::Parser::new(tokens.iter().peekable(), args.in_files[0].clone());

    let parser = match parser.start() {
        Ok(p) => p,
        Err(e) => {
            if args.verbosity >= VerboseLevel::Debug {
                return Err(e);
            } else {
                return Ok(1);
            }
        }
    };

    let ast = parser.get_ast();

    println!("{:#?}", ast);

    Ok(0)
}
