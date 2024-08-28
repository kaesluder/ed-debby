use pest::{error::Error, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "ed_command.pest"] // Adjust the grammar path as necessary
pub struct EdCommandParser;

/// Represents an address in the `ed` editor.
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
    fn from_str(input: &str) -> Option<Self> {
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

/// Parses a string input into a tuple representing a range of addresses with a separator.
///
/// # Arguments
///
/// * `input` - A string slice that holds the input to be parsed. This input should represent
/// a range in the form of two addresses separated by a comma (`,`), semicolon (`;`), or no separator.
///
/// # Returns
///
/// * `Result<(Address, RangeSep, Address), pest::error::Error<crate::ed_command_parser::Rule>>` -
///   Returns a `Result` containing a tuple with the first address, the range separator, and
///   the second address, or a `pest::error::Error` if parsing fails.
///
/// # Example
///
/// ```rust
/// let result = parse_range("10,20");
/// assert_eq!(result.unwrap(), (Address::Absolute(10), RangeSep::Comma, Address::Absolute(20)));
/// ```
///
/// # Errors
///
/// This function returns a `pest::error::Error<crate::ed_command_parser::Rule>` if the input does not
/// conform to the expected format for a range, which is defined by the `Rule::range` in the `pest` parser.
fn parse_range(
    input: &str,
) -> Result<(Address, RangeSep, Address), Error<crate::ed_command_parser::Rule>> {
    let pairs = EdCommandParser::parse(Rule::range, input)?
        .next()
        .unwrap()
        .into_inner();

    let mut address1 = Address::Current;
    let mut separator = RangeSep::Comma;
    let mut address2 = Address::None;
    let mut range_separator_present = false;

    for pair in pairs {
        match pair.as_rule() {
            Rule::address => {
                if range_separator_present {
                    address2 = Address::from_str(pair.as_str()).unwrap();
                } else {
                    address1 = Address::from_str(pair.as_str()).unwrap();
                }
            }

            Rule::range_separator => {
                range_separator_present = true;
                if pair.as_str() == ";" {
                    separator = RangeSep::Semicolon;
                }
            }

            _ => {}
        }
    }

    if (address2 == Address::None) && range_separator_present {
        address2 = Address::Current;
    } else if address2 == Address::None {
        address2 = address1.clone();
    }

    Ok((address1, separator, address2))
}

pub fn parse_line(
    input: &str,
) -> Result<EdCommand, Error<crate::ed_command_parser::Rule>> {
    let pairs = EdCommandParser::parse(Rule::line, input)?
        .next()
        .unwrap()
        .into_inner();
    let mut address1 = Address::Current;
    let mut range_sep = RangeSep::Comma;
    let mut address2 = Address::None;
    let mut command = None;
    let mut command_args = None;
    for pair in pairs {
        match pair.as_rule() {
            Rule::range => {
                (address1, range_sep, address2) = parse_range(pair.as_str())?;
            }
            Rule::command => {
                command = Some(String::from(pair.as_str()));
            }
            Rule::arg => {
                command_args = Some(String::from(pair.as_str()));
            }
            _ => (),
        }
    }
    Ok(EdCommand {
        address1,
        address2,
        range_sep,
        command,
        command_args,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // Test function definitions will go he
    #[rstest]
    #[case("50", 50, "'50' matches 50")]
    #[case("5", 5, "'5' matches 5")]
    #[case("5s/foo123/123foo/", 5, "address matcher ignores additional numbers")]
    fn test_parameterized_addresses_string(
        #[case] input: &str,
        #[case] expected: usize,
        #[case] note: &str,
    ) {
        let line = EdCommandParser::parse(Rule::address, input)
            .expect("unsuccessful parse")
            .next()
            .unwrap();

        let address = line.as_str().parse::<usize>().unwrap();
        assert_eq!(address, expected, "{}", note);
    }

    #[rstest]
    #[case("50,60", (Address::Absolute(50), RangeSep::Comma, Address::Absolute(60)), "num,num")]
    #[case("50", (Address::Absolute(50), RangeSep::Comma, Address::Absolute(50)), "num")]
    #[case("50,", (Address::Absolute(50), RangeSep::Comma, Address::Current), "num,")]
    #[case(",50", (Address::Current, RangeSep::Comma, Address::Absolute(50)), "num,")]
    #[case(".,50", (Address::Current, RangeSep::Comma, Address::Absolute(50)), "num,")]
    #[case(".,$", (Address::Current, RangeSep::Comma, Address::Last), "num,")]
    #[case("10;20", (Address::Absolute(10), RangeSep::Semicolon, Address::Absolute(20)), "num,")]
    #[case("", (Address::Current, RangeSep::Comma, Address::Current), "empty string")]
    fn test_parameterized_range_parse(
        #[case] input: &str,
        #[case] expected: (Address, RangeSep, Address),
        #[case] note: &str,
    ) -> Result<(), Error<Rule>> {
        let range = parse_range(input)?;
        assert_eq!(range, expected, "{}", note);
        Ok(())
    }

    #[rstest]
    #[case("50", Address::Absolute(50), "'50' matches 50")]
    #[case("5", Address::Absolute(5), "'5' matches 5")]
    #[case(
        "5s/foo123/123foo/",
        Address::Absolute(5),
        "address matcher ignores additional numbers"
    )]
    #[case("$", Address::Last, "match last")]
    #[case(".", Address::Current, "match current")]
    fn test_parameterized_addresses_enum(
        #[case] input: &str,
        #[case] expected: Address,
        #[case] note: &str,
    ) {
        let line = EdCommandParser::parse(Rule::address, input)
            .expect("unsuccessful parse")
            .next()
            .unwrap();

        let address = Address::from_str(line.as_str()).unwrap();
        assert_eq!(address, expected, "{}", note)
    }

    // Test function definitions will go he
    #[rstest]
    #[case("w", "w", "'w' matches 'w'")]
    #[case("q", "q", "'q' matches 'q'")]
    #[case("p", "p", "'p' matches 'p'")]
    #[case("wq", "wq", "'wq' matches 'wq'")]
    #[case("i", "i", "'i' matches 'i'")]
    #[case("=", "=", "'=' matches '='")]
    fn test_parameterized_command_parse(
        #[case] input: &str,
        #[case] expected: &str,
        #[case] note: &str,
    ) {
        let line = EdCommandParser::parse(Rule::command, input)
            .expect("unsuccessful parse")
            .next()
            .unwrap();

        let command = line.as_str();
        assert_eq!(command, expected, "{}", note);
    }
    // Test function definitions will go he
    #[rstest]
    #[case("10,15p", EdCommand{
        address1: Address::Absolute(10),
        address2: Address::Absolute(15),
        command: Some(String::from("p")),
        ..EdCommand::default()
        
    }, "print command")]
    #[case("wq", EdCommand{
        command: Some(String::from("wq")),
        ..EdCommand::default()
        
    }, "write and quit command")]
    #[case("wfoo.txt", EdCommand{
        command: Some(String::from("w")),
        command_args: Some(String::from("foo.txt")),
        ..EdCommand::default()
        
    }, "write with args")]
    #[case("1,$", EdCommand{
        address1: Address::Absolute(1),
        address2: Address::Last,
        ..EdCommand::default()
        
    }, "no command, first and last address")]
    fn test_parameterized_line_parse_to_command(
        #[case] input: &str,
        #[case] expected: EdCommand,
        #[case] note: &str,
    ) {
        let result = parse_line(input).expect("bad line parse");
        assert_eq!(result, expected, "{}", note);
    }
}
