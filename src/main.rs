mod command_parser;
use crate::command_parser::parse_args::parse_args;
use std::error::Error;
use std::env;

fn main() -> Result<(), Box<dyn Error>> {
    let _config = parse_args(env::args_os().collect())?;

    Ok(())
}
