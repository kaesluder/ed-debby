//! This module handles commands that directly modify the buffer.
//!
//! * insert: Insert lines before address.
//! * append: Append lines after address.
//! * correct: Overwrite range with new text.

use crate::buffer::line_array_buffer::LineBuffer;
use crate::ed_command_error::EdCommandError;
use crate::command_structs::{Address, EdCommand};
use crate::ed_commands::*;
use crate::input_mode::input_mode;

/// Inserts a vector of lines into the `LineBuffer` before the specified location.
///
/// This function inserts the provided lines into the buffer before the position specified
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
pub fn insert(buffer: &mut LineBuffer, command: &EdCommand) -> Result<REPLStatus, EdCommandError> {
    let input_lines = input_mode()?;

    let _index = insert_into_buffer(buffer, &command.address2, input_lines);

    Ok(REPLStatus::Continue)
}

/// Appends a vector of lines into the `LineBuffer` after the specified location.
///
/// This function appends the provided lines into the buffer after the position specified
/// by the `location` address. If the buffer is empty, the lines will be set as the
/// contents of the buffer. Otherwise, the lines are appended after the appropriate index
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
/// Returns the index of the last line that was appended.
pub fn append_into_buffer(
    buffer: &mut LineBuffer,
    location: &Address,
    lines: Vec<String>,
) -> usize {
    let mut index = address_to_index(location.clone(), buffer) + 1;
    // special case: appending to address 0 inserts *before* line 1
    if *location == Address::Absolute(0) {
        index -= index;
    }
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

/// Appends lines into the buffer after the specified address.
///
/// This function takes an `EdCommand` with an address and inserts the input lines
/// into the `LineBuffer` after the location specified by the command.
/// If a range is specified, inserts at address2. Updates current line
/// to end of inserted text.
///
/// # Arguments
///
/// * `buffer` - A mutable reference to the `LineBuffer` where the lines will be appended.
/// * `command` - A reference to the `EdCommand` containing the address.
///
/// # Returns
///
/// Returns `Ok(REPLStatus::Continue)` if the operation is successful, or an error if user input fails.
pub fn append(buffer: &mut LineBuffer, command: &EdCommand) -> Result<REPLStatus, EdCommandError> {
    let input_lines = input_mode()?;

    let _index = append_into_buffer(buffer, &command.address2, input_lines);

    Ok(REPLStatus::Continue)
}

/// Insert lines into buffer replacing the specified range.
///
/// # Arguments
///
/// * `buffer` - A mutable `LineBuffer` reference.
/// * `location` - The `Address` specifying where to insert the lines in the buffer.
/// * `lines` - A vector of strings (`Vec<String>`) containing the lines to be inserted.
///
/// # Returns
///
/// Returns the index of the last line that was appended.
pub fn correct_into_buffer(
    buffer: &mut LineBuffer,
    location1: &Address,
    location2: &Address,
    lines: Vec<String>,
) -> Result<usize, EdCommandError> {
    let index1 = address_to_index(location1.clone(), buffer);
    let index2 = address_to_index(location2.clone(), buffer);
    let input_lines_len = lines.len();
    match &mut buffer.lines {
        None => buffer.lines = Some(lines),
        Some(buffer_lines) => {
            buffer_lines.splice(index1..index2 + 1, lines);
        }
    };
    // set current line to end of inserted text.
    buffer.current_line = index2 + input_lines_len;
    Ok(buffer.current_line)
}

pub fn correct(buffer: &mut LineBuffer, command: &EdCommand) -> Result<REPLStatus, EdCommandError> {
    // handle special case where 0 is out of range
    // unlike insert and append
    if command.address1 == Address::Absolute(0) {
        eprintln!("Invalid address");
        return Err(EdCommandError::EmptyBuffer);
    }

    let input_lines = input_mode()?;
    let new_location =
        correct_into_buffer(buffer, &command.address1, &command.address2, input_lines)?;
    println!("{}", new_location);

    Ok(REPLStatus::Continue)
}

pub fn delete_from_buffer(
    buffer: &mut LineBuffer,
    location1: &Address,
    location2: &Address,
) -> Result<usize, EdCommandError> {
    let index1 = address_to_index(location1.clone(), buffer);
    let index2 = address_to_index(location2.clone(), buffer);
    match &mut buffer.lines {
        None => return Err(EdCommandError::EmptyBuffer),
        Some(buffer_lines) => {
            buffer_lines.splice(index1..index2 + 1, vec![]);
        }
    };
    // set current line to beginning of deleted range.
    buffer.current_line = index1;
    Ok(buffer.current_line)
}

pub fn delete(buffer: &mut LineBuffer, command: &EdCommand) -> Result<REPLStatus, EdCommandError> {
    // handle special case where 0 is out of range
    // unlike insert and append
    if command.address1 == Address::Absolute(0) {
        return Err(EdCommandError::InvalidRange);
    }

    let new_location = delete_from_buffer(buffer, &command.address1, &command.address2)?;
    println!("{}", new_location);

    Ok(REPLStatus::Continue)
}
#[cfg(test)]

mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    #[once]
    /// Load test file before all test functions run.
    /// This should reduce redundant file operations.
    fn test_file1() -> LineBuffer {
        let filename = "test_files/one.txt";
        LineBuffer::from_file(filename).unwrap()
    }

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

    #[rstest]
    /// Test insert into middle of buffer.
    fn test_insert_middle(test_file1: &LineBuffer) {
        // copy buffer to avoid clobbering original data
        let mut buffer = test_file1.clone();
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

    #[rstest]
    /// Test insert of empty input into buffer.
    fn test_insert_empty(test_file1: &LineBuffer) {
        // copy buffer to avoid clobbering original data
        let mut buffer = test_file1.clone();
        let address = Address::Absolute(2);
        let lines = vec![];
        let actual = insert_into_buffer(&mut buffer, &address, lines);
        assert_eq!(actual, 1);
        assert_eq!(buffer.lines.as_ref().unwrap().len(), 5);
        assert_eq!(buffer.lines.as_ref().unwrap()[1], "two".to_string())
    }

    #[rstest]
    /// Test append into middle of buffer.
    fn test_append_middle(test_file1: &LineBuffer) {
        // copy buffer to avoid clobbering original data
        let mut buffer = test_file1.clone();
        let address = Address::Absolute(2);
        let lines = vec!["alpha".to_string()];
        let actual = append_into_buffer(&mut buffer, &address, lines);
        assert_eq!(actual, 3);
        assert_eq!(buffer.lines.as_ref().unwrap().len(), 6);
        assert_eq!(buffer.lines.as_ref().unwrap()[2], "alpha".to_string())
    }

    #[rstest]
    /// Test append into start of buffer.
    fn test_append_first(test_file1: &LineBuffer) {
        // copy buffer to avoid clobbering original data
        let mut buffer = test_file1.clone();
        let address = Address::Absolute(0);
        let lines = vec!["alpha".to_string()];
        let actual = append_into_buffer(&mut buffer, &address, lines);
        assert_eq!(actual, 1);
        assert_eq!(buffer.lines.as_ref().unwrap().len(), 6);
        assert_eq!(buffer.lines.as_ref().unwrap()[0], "alpha".to_string())
    }

    #[rstest]
    /// Test append into end of buffer.
    fn test_append_last(test_file1: &LineBuffer) {
        // copy buffer to avoid clobbering original data
        let mut buffer = test_file1.clone();
        let address = Address::Last;
        let lines = vec!["alpha".to_string()];
        let actual = append_into_buffer(&mut buffer, &address, lines);
        assert_eq!(actual, 6);
        assert_eq!(buffer.lines.as_ref().unwrap().len(), 6);
        assert_eq!(buffer.lines.as_ref().unwrap()[5], "alpha".to_string())
    }

    #[rstest]
    /// Test correct into middle of buffer.
    fn test_correct_middle(test_file1: &LineBuffer) {
        // copy buffer to avoid clobbering original data
        let mut buffer = test_file1.clone();
        let address1 = Address::Absolute(2);
        let address2 = Address::Absolute(2);
        let lines = vec!["alpha".to_string()];
        let actual = correct_into_buffer(&mut buffer, &address1, &address2, lines)
            .expect("Unable to change buffer.");
        assert_eq!(actual, 2);
        assert_eq!(buffer.lines.as_ref().unwrap().len(), 5);
        assert_eq!(buffer.lines.as_ref().unwrap()[0], "one".to_string());
        assert_eq!(buffer.lines.as_ref().unwrap()[1], "alpha".to_string());
        assert_eq!(buffer.lines.as_ref().unwrap()[3], "four".to_string());
        assert_eq!(buffer.lines.as_ref().unwrap()[4], "five".to_string());
    }

    #[rstest]
    fn test_delete_middle(test_file1: &LineBuffer) {
        // copy buffer to avoid clobbering original data
        let mut buffer = test_file1.clone();
        let address1 = Address::Absolute(2);
        let address2 = Address::Absolute(2);
        let actual = delete_from_buffer(&mut buffer, &address1, &address2)
            .expect("Unable to change buffer.");
        assert_eq!(actual, 1);
        assert_eq!(buffer.lines.as_ref().unwrap().len(), 4);
        assert_eq!(buffer.lines.as_ref().unwrap()[0], "one".to_string());
        assert_eq!(buffer.lines.as_ref().unwrap()[1], "three".to_string());
        assert_eq!(buffer.lines.as_ref().unwrap()[2], "four".to_string());
        assert_eq!(buffer.lines.as_ref().unwrap()[3], "five".to_string());
    }

    #[rstest]
    fn test_delete_all(test_file1: &LineBuffer) {
        // copy buffer to avoid clobbering original data
        let mut buffer = test_file1.clone();
        let address1 = Address::Absolute(1);
        let address2 = Address::Last;
        let actual = delete_from_buffer(&mut buffer, &address1, &address2)
            .expect("Unable to change buffer.");
        assert_eq!(actual, 0);
        assert_eq!(buffer.lines.as_ref().unwrap().len(), 0);
    }

    #[rstest]
    fn test_delete_zero_returns_err(test_file1: &LineBuffer) {
        // copy buffer to avoid clobbering original data
        let mut buffer = test_file1.clone();
        let command = EdCommand {
            address1: Address::Absolute(0),
            ..EdCommand::default()
        };
        let result = delete(&mut buffer, &command);

        match result {
            Err(EdCommandError::InvalidRange) => assert!(true),
            _ => assert!(false),
        };
    }
}
