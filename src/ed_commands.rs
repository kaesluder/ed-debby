use crate::buffer::line_array_buffer::LineBuffer;
use crate::ed_command_parser::EdCommand;

use std::error::Error;

#[derive(Debug, Eq, PartialEq)]
pub enum REPLStatus {
    Continue,
    Quit,
}

pub fn command_runner(
    buffer: &mut LineBuffer,
    command: &EdCommand,
) -> Result<REPLStatus, Box<dyn Error>> {
    let repl_status = match command.command.as_deref() {
        Some("q") => quit(buffer, &command)?,
        Some("w") => write(buffer, &command)?,
        Some("wq") => write_quit(buffer, &command)?,
        _ => REPLStatus::Continue,
    };

    Ok(repl_status)
}

fn quit(_buffer: &mut LineBuffer, _command: &EdCommand) -> Result<REPLStatus, Box<dyn Error>> {
    // Perform cleanup or any necessary operations before quitting

    // Exit the program
    Ok(REPLStatus::Quit)
}

fn write(buffer: &mut LineBuffer, command: &EdCommand) -> Result<REPLStatus, Box<dyn Error>> {
    match buffer.save(command.command_args.as_deref()) {
        Ok(_) => Ok(REPLStatus::Continue),
        Err(e) => Err(Box::new(e)),
    }
}

fn write_quit(buffer: &mut LineBuffer, command: &EdCommand) -> Result<REPLStatus, Box<dyn Error>> {
    match buffer.save(command.command_args.as_deref()) {
        Ok(_) => Ok(REPLStatus::Quit),
        Err(e) => Err(Box::new(e)),
    }
}

#[cfg(test)]

mod tests {

    use super::*;
    use rstest::rstest;

    #[test]
    fn quit_returns_quit_signal() {
        let mut buffer = LineBuffer::empty();
        let command = EdCommand {
            command: Some("q".to_string()),
            ..EdCommand::default()
        };

        if let Ok(out) = command_runner(&mut buffer, &command) {
            assert_eq!(out, REPLStatus::Quit);
        } else {
            assert!(false)
        }
    }

    #[test]
    fn write_returns_continue_signal() {
        let mut buffer = LineBuffer {
            filename: Some("/tmp/foo.txt".to_string()),
            ..LineBuffer::empty()
        };
        let command = EdCommand {
            command: Some("w".to_string()),
            ..EdCommand::default()
        };

        match command_runner(&mut buffer, &command) {
            Ok(out) => assert_eq!(out, REPLStatus::Continue),
            Err(e) => {
                println!("{:?}", e);
                assert!(false);
            }
        }
        if let Ok(out) = command_runner(&mut buffer, &command) {
            assert_eq!(out, REPLStatus::Continue);
        } else {
            assert!(false)
        }
    }
    #[test]
    fn write_with_no_filename() {
        // Pass back error.
        let mut buffer = LineBuffer {
            filename: None,
            ..LineBuffer::empty()
        };
        let command = EdCommand {
            command: Some("w".to_string()),
            ..EdCommand::default()
        };

        match command_runner(&mut buffer, &command) {
            Ok(out) => assert_eq!(out, REPLStatus::Continue),
            Err(e) => {
                println!("{:?}", e);
                assert!(true);
            }
        }
    }
    #[test]
    fn write_quit_returns_quit() {
        let mut buffer = LineBuffer {
            filename: Some("/tmp/foo.txt".to_string()),
            ..LineBuffer::empty()
        };
        let command = EdCommand {
            command: Some("wq".to_string()),
            ..EdCommand::default()
        };

        match command_runner(&mut buffer, &command) {
            Ok(out) => assert_eq!(out, REPLStatus::Quit),
            Err(e) => {
                println!("{:?}", e);
                assert!(false);
            }
        }
    }
    #[test]
    fn write_quit_returns_err() {
        // Pass back error.
        let mut buffer = LineBuffer {
            filename: None,
            ..LineBuffer::empty()
        };
        let command = EdCommand {
            command: Some("w".to_string()),
            ..EdCommand::default()
        };

        match command_runner(&mut buffer, &command) {
            Ok(out) => assert_eq!(out, REPLStatus::Continue),
            Err(e) => {
                println!("{:?}", e);
                assert!(true);
            }
        }
    }
}
