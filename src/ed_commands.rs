use crate::buffer::line_array_buffer::LineBuffer;
use crate::ed_command_parser::{Address, EdCommand, EdCommandParser};
use crate::input_mode::input_mode;
use std::fmt;

use std::error::Error;

#[derive(Debug)]
pub enum EdCommandError {
    InvalidRange,
}

impl fmt::Display for EdCommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EdCommandError::InvalidRange => write!(f, "Invalid Range"),
        }
    }
}

impl std::error::Error for EdCommandError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // Since InvalidRange doesn't wrap another error, return None.
        None
    }
}

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
        Some("p") => print(buffer, &command)?,
        Some("i") => insert(buffer, &command)?,
        Some("=") => print_current_line_number(buffer, &command)?,
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

fn address_to_index(address: Address, buffer: &LineBuffer) -> usize {
    let index = match address {
        Address::Absolute(addr) => addr,
        Address::Last => buffer.len(),
        Address::Current => buffer.current_line,

        _ => buffer.current_line,
    };

    if index >= buffer.len() {
        buffer.len().saturating_sub(1)
    } else {
        index.saturating_sub(1)
    }
}

fn print(buffer: &mut LineBuffer, command: &EdCommand) -> Result<REPLStatus, Box<dyn Error>> {
    let low = address_to_index(command.address1.clone(), buffer);
    let high = address_to_index(command.address2.clone(), buffer);
    if low > high {
        return Err(Box::new(EdCommandError::InvalidRange));
    }
    match &buffer.lines {
        Some(lines) => {
            for i in low..=high {
                println!("{}", lines[i])
            }
        }
        None => println!(""),
    };
    Ok(REPLStatus::Continue)
}

fn insert_into_buffer(buffer: &mut LineBuffer, location: &Address, lines: Vec<String>) -> usize {
    let index = address_to_index(location.clone(), buffer);
    let input_lines_len = lines.len();
    match &mut buffer.lines {
        None => buffer.lines = Some(lines),
        Some(buffer_lines) => {
            buffer_lines.splice(index..index, lines);
        }
    };
    // set current line to end of inserted text.
    buffer.current_line = index + input_lines_len;
    buffer.current_line
}

fn insert(buffer: &mut LineBuffer, command: &EdCommand) -> Result<REPLStatus, Box<dyn Error>> {
    if command.address1 != command.address2 {
        return Err(Box::new(EdCommandError::InvalidRange));
    }
    let input_lines = input_mode()?;

    let _index = insert_into_buffer(buffer, &command.address2, input_lines);

    Ok(REPLStatus::Continue)
}

fn set_current_line_number(buffer: &mut LineBuffer, command: &EdCommand) {
    let index = address_to_index(command.address2.clone(), buffer);
    buffer.current_line = index + 1;
}

fn print_current_line_number(
    buffer: &mut LineBuffer,
    command: &EdCommand,
) -> Result<REPLStatus, Box<dyn Error>> {
    set_current_line_number(buffer, &command);
    println!("{}", buffer.current_line);
    Ok(REPLStatus::Continue)
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

    #[rstest]
    #[case(Address::Absolute(5), 4)]
    #[case(Address::Absolute(1000), 4)]
    #[case(Address::Absolute(0), 0)]
    #[case(Address::Current, 0)]
    #[case(Address::Last, 4)]
    fn rstest_address_to_index(#[case] address: Address, #[case] expected: usize) {
        let filename = "test_files/one.txt";
        let buffer = LineBuffer::from_file(filename).unwrap();
        let actual_index = address_to_index(address, &buffer);
        assert_eq!(actual_index, expected);
    }

    #[rstest]
    #[case(5, 4)]
    #[case(3, 2)]
    #[case(0, 0)]
    fn rstest_address_convert_mutate_current(#[case] current: usize, #[case] expected: usize) {
        let filename = "test_files/one.txt";
        let mut buffer = LineBuffer::from_file(filename).unwrap();
        buffer.current_line = current;
        let actual_index = address_to_index(Address::Current, &buffer);
        assert_eq!(actual_index, expected);
    }

    #[test]
    fn test_basic_insert() {
        let mut buffer = LineBuffer::empty();
        let address = Address::Absolute(0);
        let lines = vec!["one".to_string(), "two".to_string(), "three".to_string()];
        let actual = insert_into_buffer(&mut buffer, &address, lines);
        assert_eq!(actual, 3);
        assert_eq!(buffer.lines.unwrap()[2], "three".to_string())
    }
    #[test]
    fn test_basic_insert_middle() {
        let filename = "test_files/one.txt";
        let mut buffer = LineBuffer::from_file(filename).unwrap();
        let address = Address::Absolute(2);
        let lines = vec!["alpha".to_string()];
        let actual = insert_into_buffer(&mut buffer, &address, lines);
        assert_eq!(actual, 2);
        assert_eq!(buffer.lines.as_ref().unwrap().len(), 6);
        assert_eq!(buffer.lines.as_ref().unwrap()[1], "alpha".to_string())
    }

    #[test]
    fn test_insert_into_empty() {
        let mut buffer = LineBuffer::empty();
        let address = Address::Absolute(1);
        let lines = vec!["alpha".to_string()];
        let actual = insert_into_buffer(&mut buffer, &address, lines);
        assert_eq!(actual, 1);
        assert_eq!(buffer.lines.as_ref().unwrap().len(), 1);
        assert_eq!(buffer.lines.as_ref().unwrap()[0], "alpha".to_string())
    }
    #[test]
    fn test_insert_empty() {
        let filename = "test_files/one.txt";
        let mut buffer = LineBuffer::from_file(filename).unwrap();
        let address = Address::Absolute(2);
        let lines = vec![];
        let actual = insert_into_buffer(&mut buffer, &address, lines);
        assert_eq!(actual, 1);
        assert_eq!(buffer.lines.as_ref().unwrap().len(), 5);
        assert_eq!(buffer.lines.as_ref().unwrap()[1], "two".to_string())
    }
}
