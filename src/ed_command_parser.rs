// in your ed_command_parser.rs
use pest::Parser;
use pest_derive::Parser;

#[derive(Debug, Eq, PartialEq)]
enum Address {
    Absolute(usize),
    Last,
    Current,
}

impl Address {
    fn from_str(input: &str) -> Option<Self> {
        match input {
            "." => Some(Address::Current),
            "$" => Some(Address::Last),
            _ => input.parse::<usize>().ok().map(Address::Absolute),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum RangeSep {
    Comma,
    Semicolon,
}

#[derive(Debug, Eq, PartialEq)]
struct EdCommand {
    address1: Address,
    address2: Address,
    range_sep: RangeSep,
    command: Option<String>,
    command_args: Option<String>,
}

#[derive(Parser)]
#[grammar = "ed_command.pest"] // Adjust the grammar path as necessary
pub struct EdCommandParser;

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

    // Update your test or usage code
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
}
