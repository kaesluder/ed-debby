mod buffer;
mod command_parser;
mod ed_command_parser;
mod ed_commands;

use crate::buffer::line_array_buffer::LineBuffer;
use crate::command_parser::parse_args::parse_args;
use std::env;
use std::error::Error;

use rustyline::error::ReadlineError;
use rustyline::Editor;

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

                println!("{:#?}", command);
                let _ = ed_commands::command_runner(&mut buffer, command)?;
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
