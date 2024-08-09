use std::ffi::OsString;

const HELP: &str = "\
ed-debby

Usage: ed [options] [[+line] file]

The file name may be preceded by '+line', '+/RE', or '+?RE' to set the
current line to the line number specified or to the first or last line
matching the regular expression 'RE'.

Options:
  -h, --help                 display this help and exit
  -V, --version              output version information and exit
  -E, --extended-regexp      use extended regular expressions
  -G, --traditional          run in compatibility mode
  -l, --loose-exit-status    exit with 0 status even if a command fails
  -p, --prompt=STRING        use STRING as an interactive prompt
  -q, --quiet, --silent      suppress diagnostics written to stderr
  -r, --restricted           run in restricted mode
  -s, --script               suppress byte counts and '!' prompt
  -v, --verbose              be verbose; equivalent to the 'H' command
      --strip-trailing-cr    strip carriage returns at end of text lines
      --unsafe-names         allow control characters 1-31 in file names

Start edit by reading in 'file' if given.
If 'file' begins with a '!', read output of shell command.
";

#[derive(Debug, PartialEq, Eq)]
pub struct EdArgs {
    pub filename: Option<String>,
    pub prompt: Option<String>,
    pub verbose: bool,
    pub debug: bool,
    pub help: bool,
}

impl Default for EdArgs {
    fn default() -> Self {
        EdArgs {
            filename: None,
            prompt: None,
            verbose: false,
            debug: false,
            help: false,
        }
    }
}

pub fn parse_args(arg_list: Vec<OsString>) -> Result<EdArgs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_vec(arg_list[1..].to_vec());

    // Help has a higher priority and should be handled separately.
    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP);
        return Ok(EdArgs{help: true, ..Default::default()});
    }

    let args = EdArgs {
        prompt: pargs.opt_value_from_str(["-p", "--prompt"])?,
        debug: false,
        verbose: false,
        filename: pargs.opt_free_from_str()?,
        help: false,
    };

    // It's up to the caller what to do with the remaining arguments.
    let remaining = pargs.finish();
    if !remaining.is_empty() {
        eprintln!("Warning: unused arguments left: {:?}.", remaining);
    }

    Ok(args)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use std::ffi::OsString;

    #[test]
    fn test_help() {
        let test_args = vec!["ed", "--help"].iter().map(OsString::from).collect();
        let result = parse_args(test_args);
        assert!(result.is_ok());
    }
 
    #[test]
    fn test_filename() {
        let test_args = vec!["ed", "/tmp/foo"].iter().map(OsString::from).collect();
        let result = parse_args(test_args).expect("Error running filename test");
        if let Some(filename) = result.filename {
            assert_eq!(filename, "/tmp/foo");
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_no_args() {
        let test_args = vec!["ed"].iter().map(OsString::from).collect();
        let result = parse_args(test_args).expect("Error running filename test");
        assert!(result.filename.is_none());
    }

    #[rstest]
    // args[0] is the program name, so we don't test it.
    // add additional args in the format used by shell (separated by whitespace) 

    // long and short promt arg
    #[case(vec!["ed", "--prompt", "> "], EdArgs{prompt: Some("> ".to_string()), ..Default::default()})]
    #[case(vec!["ed", "-p", "> "], EdArgs{prompt: Some("> ".to_string()), ..Default::default()})]

    // No args is a valid case. It should return the default args.
    #[case(vec!["ed"], EdArgs{..Default::default()})]

    // filename and prompt args
    #[case(vec!["ed", "/tmp/foo"], EdArgs{filename: Some("/tmp/foo".to_string()), ..Default::default()})]
    #[case(vec!["ed", "/tmp/foo", "--prompt", "> "], 
        EdArgs{filename: Some("/tmp/foo".to_string()), 
                prompt: Some("> ".to_string()), ..Default::default()})] 
    fn parameterized_cli_arg_test(#[case] args: Vec<&str>, #[case] expected: EdArgs) {
        let args = args.iter().map(OsString::from).collect();
        let result = parse_args(args).expect("Error running prompt test");
        assert_eq!(result, expected);
    }
}
