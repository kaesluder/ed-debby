use crate::ed_command_parser::Rule;
use std::fmt;


/// Represents errors that can occur when executing an editor command.
///
/// # Variants
///
/// * `InvalidRange` - Indicates that the specified range in the command is invalid, such as when the first address is greater than the second or the address is out of bounds.
/// * `EmptyBuffer` - Indicates that an operation was attempted on an empty buffer.
#[derive(Debug)]
pub enum EdCommandError {
    InvalidRange,
    EmptyBuffer,
    InputModeError(rustyline::error::ReadlineError),
    ParseError(Box<pest::error::Error<Rule>>),
}

/// Automatically wrap ReadLineError in an EdCommandError
/// allows for using `?` for error handling in input_mode
impl From<rustyline::error::ReadlineError> for EdCommandError {
    fn from(err: rustyline::error::ReadlineError) -> EdCommandError {
        EdCommandError::InputModeError(err)
    }
}

impl fmt::Display for EdCommandError {
    /// Formats the error message for display.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EdCommandError::InvalidRange => write!(f, "Invalid Range"),
            EdCommandError::EmptyBuffer => write!(f, "Empty Buffer"),
            EdCommandError::InputModeError(ref e) => write!(f, "Input Error: {}", e),
            EdCommandError::ParseError(ref e) => write!(f, "Parse Error: {}", e),
        }
    }
}

impl std::error::Error for EdCommandError {
    /// Returns the source of the error, if any. In this case, no underlying error is wrapped, so it returns `None`.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            EdCommandError::ParseError(ref e) => Some(e),
            _ => None,
        }
    }
}

