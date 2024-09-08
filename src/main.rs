mod buffer;
mod command_parser;
mod ed_command_parser;
mod ed_commands;
mod input_mode;
mod modify;
mod ed_command_error;
mod command_structs;

use crate::buffer::line_array_buffer::LineBuffer;
use crate::command_parser::parse_args::parse_args;
use std::env;
use std::error::Error;

use ed_commands::REPLStatus;
use rustyline::error::ReadlineError;

fn main() -> Result<(), Box<dyn Error>> {
    let config = parse_args(env::args_os().collect())?;
    let mut rl = rustyline::DefaultEditor::new()?;
    let prompt = config.prompt.as_deref().unwrap_or("");

    if config.help {
        return Ok(());
    }

    let mut buffer = if let Some(filename) = config.filename {
        LineBuffer::from_file(&filename)?
    } else {
        LineBuffer::empty()
    };

    loop {
        let readline = rl.readline(prompt);

        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                let command = ed_command_parser::parse_line(line.as_str())?;
                if config.diagnostics {
                    println!("{:#?}", command);
                }
                let result_or_err = ed_commands::command_runner(&mut buffer, &command);
                let result = match result_or_err {
                    Ok(result) => result,
                    Err(e) => {
                        eprintln!("{}", e);
                        continue;
                    }
                };

                if result == REPLStatus::Quit {
                    break;
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
