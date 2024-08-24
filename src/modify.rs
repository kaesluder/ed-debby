//! This module handles commands that directly modify the buffer.
//!
//! * insert: Insert lines before address.

use crate::buffer::line_array_buffer::LineBuffer;
use crate::ed_command_parser::{Address, EdCommand};
use crate::ed_commands::*;
use crate::input_mode::input_mode;
use std::error::Error;

/// Inserts a vector of lines into the `LineBuffer` at the specified location.
///
/// This function inserts the provided lines into the buffer at the position specified
/// by the `location` address. If the buffer is empty, the lines will be set as the
/// contents of the buffer. Otherwise, the lines are inserted before the appropriate index
/// without replacing any existing lines. Sets `buffer.current_line` to the end of
/// the insert.
///
/// # Arguments
///
/// * `buffer` - A mutable `LineBuffer` reference.
/// * `location` - The `Address` specifying where to insert the lines in the buffer.
/// * `lines` - A vector of strings (`Vec<String>`) containing the lines to be inserted.
///
/// # Returns
///
/// Returns the index of the last line that was inserted.
///
/// # Example
///
/// ```
/// let mut buffer = LineBuffer::new();
/// let location = Address::Absolute(3);
/// let lines = vec![String::from("line1"), String::from("line2")];
/// let current_line = insert_into_buffer(&mut buffer, &location, lines);
/// assert_eq!(current_line, 4);
/// ```
pub fn insert_into_buffer(
    buffer: &mut LineBuffer,
    location: &Address,
    lines: Vec<String>,
) -> usize {
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

/// Inserts lines into the buffer at the specified address.
///
/// This function takes an `EdCommand` with an address and inserts the input lines
/// into the `LineBuffer` before the location specified by the command.
/// If a range is specified, inserts at address2. Updates current line
/// to end of inserted text.
///
/// # Arguments
///
/// * `buffer` - A mutable reference to the `LineBuffer` where the lines will be inserted.
/// * `command` - A reference to the `EdCommand` containing the address and other command details.
///
/// # Returns
///
/// Returns `Ok(REPLStatus::Continue)` if the operation is successful, or an error if user input fails.
pub fn insert(buffer: &mut LineBuffer, command: &EdCommand) -> Result<REPLStatus, Box<dyn Error>> {
    let input_lines = input_mode()?;

    let _index = insert_into_buffer(buffer, &command.address2, input_lines);

    Ok(REPLStatus::Continue)
}
pub fn append_into_buffer(
    buffer: &mut LineBuffer,
    location: &Address,
    lines: Vec<String>,
) -> usize {
    let index = address_to_index(location.clone(), buffer) + 1;
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

#[cfg(test)]

mod tests {
    use super::*;
    use rstest::rstest;
    #[test]
    /// Test basic insert at start of buffer.
    fn test_basic_insert() {
        let mut buffer = LineBuffer::empty();
        let address = Address::Absolute(0);
        let lines = vec!["one".to_string(), "two".to_string(), "three".to_string()];
        let actual = insert_into_buffer(&mut buffer, &address, lines);
        assert_eq!(actual, 3);
        assert_eq!(buffer.lines.unwrap()[2], "three".to_string())
    }
    #[test]
    /// Test insert into middle of buffer.
    fn test_insert_middle() {
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
    /// Test insert into empty buffer.
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
    /// Test insert of empty input into buffer.
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

    #[test]
    /// Test insert into middle of buffer.
    fn test_append_middle() {
        let filename = "test_files/one.txt";
        let mut buffer = LineBuffer::from_file(filename).unwrap();
        let address = Address::Absolute(2);
        let lines = vec!["alpha".to_string()];
        let actual = append_into_buffer(&mut buffer, &address, lines);
        assert_eq!(actual, 3);
        assert_eq!(buffer.lines.as_ref().unwrap().len(), 6);
        assert_eq!(buffer.lines.as_ref().unwrap()[2], "alpha".to_string())
    }
}
