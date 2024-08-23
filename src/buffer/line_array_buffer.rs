#[derive(Debug, PartialEq, Eq)]
pub struct LineBuffer {
    pub lines: Option<Vec<String>>,
    pub filename: Option<String>,
    pub current_line: usize,
}

impl LineBuffer {
    pub fn empty() -> Self {
        LineBuffer {
            filename: None,
            lines: None,
            current_line: 0,
        }
    }

    // Constructor to create LineBuffer from a file
    pub fn from_file(filename: &str) -> Result<Self, std::io::Error> {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

        Ok(LineBuffer {
            filename: Some(filename.to_string()),
            lines: Some(lines),
            current_line: 0,
        })
    }

    // Save the lines to a file
    pub fn save(&mut self, filename: Option<&str>) -> Result<(), std::io::Error> {
        use std::io::Write;
        let filename = match filename {
            Some(f) => {
                self.filename = Some(f.to_string());
                f.to_string()
            }
            None => match &self.filename {
                Some(f) => f.clone(),
                None => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "No filename provided",
                    ))
                }
            },
        };

        let mut file = std::fs::File::create(&filename)?;

        if let Some(lines) = &self.lines {
            for line in lines {
                writeln!(file, "{}", line)?;
            }
        }

        Ok(())
    }

    pub fn char_length(&self) -> Option<usize> {
        if let Some(lines) = &self.lines {
            let total_length = lines
                .iter()
                .map(|s| s.len())
                .reduce(|acc, sl| acc + sl)
                .unwrap_or(0);

            // Adding the number of lines to the total length
            Some(total_length + lines.len())
        } else {
            None
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_empty_buffer_create() {
        let buff = LineBuffer::empty();
        assert!(buff.lines == None);
        assert!(buff.filename == None);
    }

    #[test]
    fn test_from_file_create() {
        let filename = "test_files/one.txt";
        let buff = LineBuffer::from_file(filename).unwrap();
        assert!(buff.filename == Some(filename.to_string()));
        assert!(buff.lines.is_some());
        if let Some(local_lines) = buff.lines {
            assert!(local_lines[0] == "one".to_string());
            assert!(local_lines[4] == "five".to_string());
        }
    }

    #[test]
    fn test_file_save() {
        let filename = "test_files/one.txt";
        let mut buff = LineBuffer::from_file(filename).unwrap();

        let out_filename = "/tmp/out.txt";
        buff.save(Some(out_filename)).unwrap();

        let saved_buff = LineBuffer::from_file(out_filename).unwrap();

        assert_eq!(
            buff.lines, saved_buff.lines,
            "The saved buffer does not match the original buffer."
        );
    }

    #[test]
    fn test_count_chars() {
        let filename = "test_files/one.txt";
        let buff = LineBuffer::from_file(filename).unwrap();
        assert_eq!(buff.char_length(), Some(24), "Char count should be 24");
    }

    #[test]
    fn test_count_chars_empty() {
        let buff = LineBuffer::empty();
        assert_eq!(
            buff.char_length(),
            None,
            "Char count for empty buffer should be none."
        );
    }
}
