mod command_parser;
use crate::command_parser::parse_args::parse_args;
use std::error::Error;
use std::env;

fn main() -> Result<(), Box<dyn Error>> {
    let config = parse_args(env::args_os().collect())?;

    if config.help {
        return Ok(());
    }

    Ok(())
}
