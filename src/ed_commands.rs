use crate::buffer::line_array_buffer::LineBuffer;
use crate::ed_command_parser::{Address, EdCommand};
use crate::modify::*;
use crate::ed_command_error::EdCommandError;

use std::error::Error;

/// Signal for REPL to `Continue` or `Quit`.
///
/// # Variants
///
/// * `Continue` - Indicates that the REPL should continue running after the current command.
/// * `Quit` - Indicates that the REPL should exit.
#[derive(Debug, Eq, PartialEq)]
pub enum REPLStatus {
    Continue,
    Quit,
}

/// Validates that addresses provided with command are within buffer bounds and in the correct order.
/// `Absolute(0) <= command.address1 <= buffer.current <= buffer.len()`
///
/// # Arguments
///
/// * `buffer` - A reference to the `LineBuffer`, which holds the lines of text being edited.
/// * `command` - A reference to the `EdCommand`, which contains the addresses that need to be validated.
///
/// # Return Value
///
/// Returns `Result<(), EdCommandError>`, where:
/// * `Ok(())` indicates that the range is valid.
/// * An `EdCommandError::InvalidRange` error is returned if the addresses specified in the command are out of bounds or the first address is greater than the second address.
fn validate_range(buffer: &mut LineBuffer, command: &EdCommand) -> Result<(), EdCommandError> {
    if buffer.current_line > buffer.len() {
        buffer.current_line = buffer.len();
    }

    // println!("{}, {}", buffer.current_line, buffer.len());

    let address1 = match command.address1.clone() {
        Address::Current => Address::Absolute(buffer.current_line),
        Address::Last => Address::Absolute(buffer.len()),
        other => other,
    };

    let address2 = match command.address2.clone() {
        Address::Current => Address::Absolute(buffer.current_line),
        Address::Last => Address::Absolute(buffer.len()),
        other => other,
    };

    if address1 > address2 {
        return Err(EdCommandError::InvalidRange);
    }

    // Check if address1 is within the valid range
    if let Address::Absolute(addr) = address1 {
        if addr > buffer.len() {
            return Err(EdCommandError::InvalidRange);
        }
    }

    // Check if address2 is within the valid range
    if let Address::Absolute(addr) = address2 {
        if addr > buffer.len() {
            return Err(EdCommandError::InvalidRange);
        }
    }
    Ok(())
}

/// Executes the given command on the buffer and returns the result status.
///
/// # Arguments
///
/// * `buffer` - A mutable reference to the `LineBuffer`, which holds the lines of text being edited.
/// * `command` - A reference to the `EdCommand`, which specifies the command to be executed and its associated parameters.
///
/// # Return Value
///
/// Returns `Result<REPLStatus, Box<dyn Error>>` where:
/// * `Ok(REPLStatus::Continue)` indicates that the command was executed successfully and the REPL should continue running.
/// * `Ok(REPLStatus::Quit)` indicates that the editor should exit.
/// * An error is returned if the command execution fails, such as due to an invalid range or file I/O error.
pub fn command_runner(
    buffer: &mut LineBuffer,
    command: &EdCommand,
) -> Result<REPLStatus, Box<dyn Error>> {
    validate_range(buffer, &command)?;
    let repl_status = match command.command.as_deref() {
        Some("q") => quit(buffer, &command)?,
        Some("w") => write(buffer, &command)?,
        Some("wq") => write_quit(buffer, &command)?,
        Some("p") => print(buffer, &command)?,
        Some("i") => insert(buffer, &command)?,
        Some("=") => print_current_line_number(buffer, &command)?,
        Some("a") => append(buffer, &command)?,
        Some("c") => correct(buffer, &command)?,
        Some("d") => delete(buffer, &command)?,
        Some("n") => print_with_numbers(buffer, &command)?,
        _ => REPLStatus::Continue,
    };

    Ok(repl_status)
}

/// Quits the editor, performing any necessary cleanup before exiting.
///
/// # Arguments
///
/// * `_buffer` - An unused mutable reference to the `LineBuffer`.
/// * `_command` - An unused reference to the `EdCommand`.
///
/// # Return Value
///
/// Returns `Result<REPLStatus, Box<dyn Error>>` with `Ok(REPLStatus::Quit)` indicating the editor should exit.
fn quit(_buffer: &mut LineBuffer, _command: &EdCommand) -> Result<REPLStatus, Box<dyn Error>> {
    // Perform cleanup or any necessary operations before quitting

    // Exit the program
    Ok(REPLStatus::Quit)
}

/// Writes the buffer to a file and continues editing.
///
/// # Arguments
///
/// * `buffer` - A mutable reference to the `LineBuffer`, which holds the lines of text to be saved.
/// * `command` - A reference to the `EdCommand`, containing any arguments for the write operation (e.g., file name).
///
/// # Return Value
///
/// Returns `Result<REPLStatus, Box<dyn Error>>` with `Ok(REPLStatus::Continue)` if the buffer is successfully saved, or an error if the save operation fails.
fn write(buffer: &mut LineBuffer, command: &EdCommand) -> Result<REPLStatus, Box<dyn Error>> {
    match buffer.save(command.command_args.as_deref()) {
        Ok(_) => Ok(REPLStatus::Continue),
        Err(e) => Err(Box::new(e)),
    }
}

/// Writes the buffer to a file and then quits the editor.
///
/// # Arguments
///
/// * `buffer` - A mutable reference to the `LineBuffer`, which holds the lines of text to be saved.
/// * `command` - A reference to the `EdCommand`, containing any arguments for the write operation (e.g., file name).
///
/// # Return Value
///
/// Returns `Result<REPLStatus, Box<dyn Error>>` with `Ok(REPLStatus::Quit)` if the buffer is successfully saved, or an error if the save operation fails.
fn write_quit(buffer: &mut LineBuffer, command: &EdCommand) -> Result<REPLStatus, Box<dyn Error>> {
    match buffer.save(command.command_args.as_deref()) {
        Ok(_) => Ok(REPLStatus::Quit),
        Err(e) => Err(Box::new(e)),
    }
}

pub fn address_to_index(address: Address, buffer: &LineBuffer) -> usize {
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

/// Prints the lines within the specified range in the buffer.
///
/// # Arguments
///
/// * `buffer` - A mutable reference to the `LineBuffer`, which holds the lines of text being edited.
/// * `command` - A reference to the `EdCommand`, containing the addresses specifying the range of lines to print.
///
/// # Result
///
/// Returns `Result<REPLStatus, Box<dyn Error>>`, where `Ok(REPLStatus::Continue)` indicates successful execution.
/// Returns an `EdCommandError::EmptyBuffer` error if the buffer is empty, or an `EdCommandError::InvalidRange` error if the specified range is invalid.
fn print(buffer: &mut LineBuffer, command: &EdCommand) -> Result<REPLStatus, Box<dyn Error>> {
    if buffer.len() == 0 {
        return Err(Box::new(EdCommandError::EmptyBuffer));
    }
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

/// Prints the lines within the specified range from the `LineBuffer`
/// each line prefixed with its line number.
/// If the buffer is empty or the specified range is invalid,
/// it returns an appropriate error.
///
/// # Arguments
///
/// * `buffer` - A reference to the `LineBuffer`, which holds the lines of text being edited.
/// * `command` - A reference to the `EdCommand`, containing the addresses that specify the range of lines to print.
///
/// # Returns
///
/// * `Result<REPLStatus, Box<dyn Error>>` - Returns `Ok(REPLStatus::Continue)` on success, otherwise an error wrapped in a `Box<dyn Error>`.
fn print_with_numbers(
    buffer: &LineBuffer,
    command: &EdCommand,
) -> Result<REPLStatus, Box<dyn Error>> {
    if buffer.len() == 0 {
        return Err(Box::new(EdCommandError::EmptyBuffer));
    }
    let low = address_to_index(command.address1.clone(), buffer);
    let high = address_to_index(command.address2.clone(), buffer);
    if low > high {
        return Err(Box::new(EdCommandError::InvalidRange));
    }
    match &buffer.lines {
        Some(lines) => {
            for i in low..=high {
                println!("{:>4}\t{}", i + 1, lines[i]);
            }
        }
        None => println!(""),
    };
    Ok(REPLStatus::Continue)
}

/// Sets the line number from the `command` on the `buffer` object.
fn set_current_line_number(buffer: &mut LineBuffer, command: &EdCommand) {
    let index = address_to_index(command.address2.clone(), buffer);
    buffer.current_line = index + 1;
}

/// Prints the current line number of the buffer.
///
/// # Arguments
///
/// * `buffer` - A mutable reference to the `LineBuffer`, which holds the lines of text being edited.
/// * `command` - A reference to the `EdCommand`, representing the command to be executed.
///
/// ## Returns
///
/// `Result<REPLStatus>`
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
    fn test_print_with_numbers_empty_buffer() {
        let buffer = LineBuffer::empty();
        let command = EdCommand {
            command: Some("p".to_string()),
            address1: Address::Absolute(1),
            address2: Address::Absolute(1),
            ..EdCommand::default()
        };

        let result = print_with_numbers(&buffer, &command);

        assert!(result.is_err());
        if let Err(ref e) = result {
            assert_eq!(format!("{}", e), format!("{}", EdCommandError::EmptyBuffer));
        }
    }

    #[test]
    fn test_print_with_numbers_invalid_range() {
        let buffer = LineBuffer {
            lines: Some(vec![
                "line one".to_string(),
                "line two".to_string(),
                "line three".to_string(),
            ]),
            ..LineBuffer::empty()
        };

        let command = EdCommand {
            command: Some("p".to_string()),
            address1: Address::Absolute(3),
            address2: Address::Absolute(1),
            ..EdCommand::default()
        };

        let result = print_with_numbers(&buffer, &command);

        assert!(result.is_err());
        if let Err(ref e) = result {
            assert_eq!(
                format!("{}", e),
                format!("{}", EdCommandError::InvalidRange)
            );
        }
    }
}
