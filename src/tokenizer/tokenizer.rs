#[derive(Debug)]
pub struct CsvTokenizer<'a> {
    input: &'a str,
    position: usize,
    delimiter: char,
}

#[derive(Debug, PartialEq)]
pub enum CsvToken<'a> {
    Field(&'a str),
    Delimiter,
    Newline,
}

impl<'a> CsvTokenizer<'a> {
    pub fn new(input: &'a str, delimiter: char) -> CsvTokenizer<'a> {
        CsvTokenizer {
            input,
            delimiter,
            position: 0,
        }
    }
}

impl<'a> Iterator for CsvTokenizer<'a> {
    type Item = CsvToken<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let token = match self.current() {
            Some(b'\n') => {
                self.position += 1;
                Some(CsvToken::Newline)
            }
            Some(char) => {
                if *char == self.delimiter as u8 {
                    self.position += 1;
                    Some(CsvToken::Delimiter)
                } else {
                    let length = self.chars_until_end_of_field();
                    let result = Some(CsvToken::Field(
                        &self.input[self.position..self.position + length],
                    ));
                    self.position += length;
                    result
                }
            }
            None => None,
        };
        token
    }
}

impl CsvTokenizer<'_> {
    fn current(&self) -> Option<&u8> {
        self.input.as_bytes().get(self.position)
    }

    fn get_offset(&self, i: usize) -> Option<&u8> {
        self.input.as_bytes().get(self.position + i)
    }

    fn chars_until_end_of_field(&self) -> usize {
        let mut i = 1;
        loop {
            match self.get_offset(i) {
                None => break,
                Some(byte) => {
                    if *byte as char == self.delimiter || *byte == b'\n' {
                        break;
                    }
                }
            }
            i += 1;
        }
        i
    }
}

#[test]
fn it_handles_empty_input() {
    let mut t = CsvTokenizer::new("", ',');
    assert_eq!(t.next(), None);
}

#[test]
fn it_handles_single_char_fields_with_newlines_and_delimiters() {
    let mut t = CsvTokenizer::new("1,2,3\n4,5,6", ',');
    assert_eq!(t.next(), Some(CsvToken::Field("1")));
    assert_eq!(t.next(), Some(CsvToken::Delimiter));
    assert_eq!(t.next(), Some(CsvToken::Field("2")));
    assert_eq!(t.next(), Some(CsvToken::Delimiter));
    assert_eq!(t.next(), Some(CsvToken::Field("3")));
    assert_eq!(t.next(), Some(CsvToken::Newline));
    assert_eq!(t.next(), Some(CsvToken::Field("4")));
    assert_eq!(t.next(), Some(CsvToken::Delimiter));
    assert_eq!(t.next(), Some(CsvToken::Field("5")));
    assert_eq!(t.next(), Some(CsvToken::Delimiter));
    assert_eq!(t.next(), Some(CsvToken::Field("6")));
    assert_eq!(t.next(), None);
}

#[test]
fn it_handles_multiple_char_fields() {
    let mut t = CsvTokenizer::new("abc,def", ',');
    assert_eq!(t.next(), Some(CsvToken::Field("abc")));
    assert_eq!(t.next(), Some(CsvToken::Delimiter));
    assert_eq!(t.next(), Some(CsvToken::Field("def")));
    assert_eq!(t.next(), None);
}
