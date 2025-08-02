use std::env;
use std::fs;
use std::io;

#[derive(Debug, PartialEq)]
struct Csv {
    lines: Vec<CsvLine>,
}

#[derive(Debug, PartialEq)]
struct CsvLine {
    values: Vec<String>,
}

impl From<String> for Csv {
    fn from(val: String) -> Self {
        let items: Vec<CsvLine> = val.trim().lines().map(Csv::line_to_vec).collect();

        Csv { lines: items }
    }
}

impl From<&str> for Csv {
    fn from(value: &str) -> Self {
        String::from(value).into()
    }
}

impl PartialEq<Vec<&str>> for CsvLine {
    fn eq(&self, other: &Vec<&str>) -> bool {
        self.values == *other
    }
}

impl Csv {
    fn line_to_vec(line: &str) -> CsvLine {
        let values: Vec<String> = line
            .split(',')
            .map(|x| x.trim_matches('"').to_owned())
            .collect();
        CsvLine { values }
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let raw_csv = fs::read_to_string(path)?;
    let csv: Csv = raw_csv.into();

    dbg!(csv);

    Ok(())
}

#[test]
fn it_parses_csv_with_strings() {
    let input = "head1,head2,head3
val1,val2,val3";

    let csv: Csv = input.into();

    assert_eq!(
        csv.lines,
        vec![
            vec!["head1", "head2", "head3"],
            vec!["val1", "val2", "val3"]
        ]
    );
}

#[test]
fn it_parses_csv_with_quoted_values() {
    let input = r#"head1","head2","head3"
"val1","val2","val3""#;

    let csv: Csv = input.into();

    assert_eq!(
        csv.lines,
        vec![
            vec!["head1", "head2", "head3"],
            vec!["val1", "val2", "val3"]
        ]
    );
}
