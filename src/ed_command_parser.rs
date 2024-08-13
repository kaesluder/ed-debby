use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "ed_command.pest"] // Adjust the grammar path as necessary
pub struct EdCommandParser;

#[derive(Debug, Eq, PartialEq, Clone)]
enum Address {
    Absolute(usize),
    Last,
    Current,
    None,
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

fn parse_range(input: &str) -> (Address, RangeSep, Address) {
    let pairs = EdCommandParser::parse(Rule::range, input)
        .expect("unsuccessful parse")
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

    (address1, separator, address2)
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
    fn test_parameterized_range_parse(
        #[case] input: &str,
        #[case] expected: (Address, RangeSep, Address),
        #[case] note: &str,
    ) {
        let range = parse_range(input);
        assert_eq!(range, expected, "{}", note)
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
}
