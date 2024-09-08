/// Represents an address in the `ed` editor.
/// 
///
/// # Values
///
/// * `Abolute(usize)` - An absolute one-indexed linenumber reference.
/// * `Last` - The last line in the buffer (`$`).
/// * `Current` - Current line in buffer (`.`, default for most cases).
/// * `None` - No address.
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub enum Address {
    Current,
    Absolute(usize),
    Last,
    None,
}

impl Address {
    /// Parses a string representing an `ed` address, returning an `Address` enum.
    ///
    /// # Arguments
    ///
    /// * `input` - string slice of address from `ed` command string
    ///
    /// # Returns
    ///
    /// * `Option<Address>` - Returns `Option` containing an `Address` enum. `None` if string cannot be parsed to `usize`.
    ///
    pub fn from_str(input: &str) -> Option<Self> {
        match input {
            "." => Some(Address::Current),
            "$" => Some(Address::Last),
            _ => input.parse::<usize>().ok().map(Address::Absolute), // returns `None` on parse error
        }
    }
}

/// Represents an `ed` range separator.
///
/// # Values
///
/// * `Comma` - Preserves current line.
/// * `Semicolon` - Sets current line to first value in range.
#[derive(Debug, Eq, PartialEq)]
pub enum RangeSep {
    Comma,
    Semicolon,
}

/// Represents an ed command with optional addresses, a range separator,
/// and an optional command with arguments.
///
/// The `EdCommand` struct is used to represent a parsed command in an `ed`-style text editor.
/// It includes two addresses that specify a range, a separator between those addresses,
/// and an optional command with associated arguments.
///
/// # Fields
///
/// * `address1` - The first address in the command. This can represent the starting point of a range or a single address.
/// * `address2` - The second address in the command. This can represent the end point of a range.
/// * `range_sep` - The separator used between the two addresses, typically a comma (`,`) or semicolon (`;`).
/// * `command` - An optional `String` representing the command to be executed.
/// * `command_args` - An optional `String` representing the arguments to the command.
///
/// # Example
///
/// ```rust
/// let cmd = EdCommand {
///     address1: Address::Absolute(1),
///     address2: Address::Absolute(5),
///     range_sep: RangeSep::Comma,
///     command: Some("d".to_string()),
///     command_args: None,
/// };
/// ```
#[derive(Debug, Eq, PartialEq)]
pub struct EdCommand {
    pub address1: Address,
    pub address2: Address,
    pub range_sep: RangeSep,
    pub command: Option<String>,
    pub command_args: Option<String>,
}

impl EdCommand {
    pub fn default() -> EdCommand {
        EdCommand {
            address1: Address::Current,
            address2: Address::Current,
            range_sep: RangeSep::Comma,
            command: None,
            command_args: None,
        }
    }
}
