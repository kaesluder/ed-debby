#[cfg(test)]

struct EdAddress {
    top: Option<usize>,
    bottom: Option<usize>,
}



mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("1", 
        EdAddress{top: Some(1), bottom: None}, 
        "Single number '1'")]     
    fn parameterized_cli_arg_test(#[case] address_str: &str, #[case] expected: EdAddress, #[case] message: &str) {
        let result: EdAddress = parse_address(address_str);
        assert_eq!(result, expected, "{}", message);
    }
    
}

