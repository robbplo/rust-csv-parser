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
        let bytes = self.input.as_bytes();
        let token = match bytes.get(self.position) {
            Some(b'\n') => Some(CsvToken::Newline),
            Some(char) => {
                if *char == self.delimiter as u8 {
                    Some(CsvToken::Delimiter)
                } else {
                    Some(CsvToken::Field(
                        &self.input[self.position..self.position + 1],
                    ))
                }
            }
            None => None,
        };
        self.position += 1;
        token
    }
}

#[test]
fn it_handles_empty_input() {
    let mut t = CsvTokenizer::new("", ',');
    assert_eq!(t.next(), None);
}

#[test]
fn it_handles_single_char_fields_and_delimiters() {
    let mut t = CsvTokenizer::new("1,2,3", ',');
    assert_eq!(t.next(), Some(CsvToken::Field("1")));
    assert_eq!(t.next(), Some(CsvToken::Delimiter));
    assert_eq!(t.next(), Some(CsvToken::Field("2")));
    assert_eq!(t.next(), Some(CsvToken::Delimiter));
    assert_eq!(t.next(), Some(CsvToken::Field("3")));
    assert_eq!(t.next(), None);
}
