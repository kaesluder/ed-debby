use crate::ed_commands::EdCommandError;
use rustyline::error::ReadlineError;
use std::error::Error;

pub fn input_mode() -> Result<Vec<String>, Box<dyn Error>> {
    let mut rl = rustyline::DefaultEditor::new()?;

    let mut input_buffer: Vec<String> = vec![];

    loop {
        let readline = rl.readline("");

        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                match line.trim() {
                    "." => {
                        return Ok(input_buffer);
                    }
                    _ => {
                        input_buffer.push(line);
                    }
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

    Ok(input_buffer)
}
