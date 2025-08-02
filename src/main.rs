use std::env;
use std::fs;
use std::io;

#[derive(Debug)]
enum CsvValue {
    String(String),
    Integer(i64),
    Float(f64),
    Null,
}

#[derive(Debug)]
struct Csv {
    header: Vec<String>,
    items: Vec<Vec<String>>,
}

impl From<String> for Csv {
    fn from(val: String) -> Self {
        let mut lines = val.trim().lines();
        let header: Vec<String> = Csv::line_to_vec(lines.next().expect("no first line"));
        let items: Vec<Vec<String>> = lines.map(|x| Csv::line_to_vec(x)).collect();

        Csv { header, items }
    }
}

impl From<&str> for Csv {
    fn from(value: &str) -> Self {
        String::from(value).into()
    }
}

impl Csv {
    fn line_to_vec(line: &str) -> Vec<String> {
        line.split(',').map(|x| x.to_owned()).collect()
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

    assert_eq!(csv.header, vec!["head1", "head2", "head3"]);
    assert_eq!(csv.items, vec![vec!["val1", "val2", "val3"]]);
}
