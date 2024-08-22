use crate::buffer::line_array_buffer::LineBuffer;
use crate::ed_command_parser::EdCommand;

use std::error::Error;

pub fn command_runner(buffer: &mut LineBuffer, command: EdCommand) -> Result<(), Box<dyn Error>> {
    match command.command.as_deref() {
        Some("q") => {
            quit(buffer, command)?;
        }
        _ => (),
    }

    Ok(())
}

fn quit(_buffer: &mut LineBuffer, _command: EdCommand) -> Result<(), Box<dyn Error>> {
    // Perform cleanup or any necessary operations before quitting

    // Exit the program
    std::process::exit(0);
}
