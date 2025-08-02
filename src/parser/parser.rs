use core::panic;

use crate::tokenizer::{self, CsvToken, CsvTokenizer};

#[derive(Debug, PartialEq)]
struct Csv {
    lines: Vec<CsvLine>,
}

#[derive(Debug, PartialEq)]
struct CsvLine {
    values: Vec<String>,
}

#[derive(Debug)]
pub struct CsvParser<'a> {
    tokenizer: CsvTokenizer<'a>,
}

impl CsvParser<'_> {
    pub fn new(tokenizer: CsvTokenizer) -> CsvParser {
        CsvParser { tokenizer }
    }

    // pub fn parse_string(&self, input: &str) -> Csv {
    //     let mut lines: Vec<CsvLine> = vec![];
    // }

    pub fn get_line(&mut self) -> CsvLine {
        let values: Vec<String> = self
            .tokenizer
            .by_ref()
            .into_iter()
            .filter(|x| *x != CsvToken::Delimiter)
            .take_while(|x| *x != CsvToken::Newline)
            .map(|x| match x {
                CsvToken::Field(string) => string.to_owned(),
                _ => panic!("expected only field tokens"),
            })
            .collect();

        CsvLine { values }
    }
}

#[test]
fn it_gets_lines_from_csv() {
    let input = "abc,def\n123,456";
    let mut parser = CsvParser::new(CsvTokenizer::new(input, ','));
    let line = parser.get_line();
    assert_eq!(line.values, vec!["abc", "def"]);
    let line = parser.get_line();
    assert_eq!(line.values, vec!["123", "456"]);
}
