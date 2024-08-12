// in your ed_command_parser.rs
use pest::Parser;
use pest_derive::Parser;

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
    fn test_parameterized_addresses(
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
}
