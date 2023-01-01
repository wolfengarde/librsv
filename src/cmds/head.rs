use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use tabled::builder::Builder;
use tabled::Style;

pub fn run(
    filename: &str,
    no_header: bool,
    sep: &str,
    n: usize,
    tabled: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // current file
    let mut path = std::env::current_dir()?;
    path.push(Path::new(filename));

    // show head n
    let r = BufReader::new(File::open(path)?)
        .lines()
        .take(n + 1 - no_header as usize)
        .filter_map(|i| i.ok())
        .collect::<Vec<_>>();

    // tabled or not
    match tabled {
        true => {
            print_as_table(r, sep, no_header);
        }
        false => {
            r.iter().for_each(|i| println!("{}", i));
        }
    }

    Ok(())
}

fn print_as_table(records: Vec<String>, sep: &str, no_header: bool) {
    let mut rdr = records.iter();
    let mut builder = Builder::default();

    // header
    if !no_header {
        match rdr.next() {
            Some(row) => {
                let col = row.split(sep).collect::<Vec<_>>();
                builder.set_columns(col);
            }
            None => {}
        }
    }

    // content
    while let Some(row) = rdr.next() {
        let r = row.split(sep).collect::<Vec<_>>();
        builder.add_record(r);
    }

    // build
    let mut table = builder.build();

    // style
    table.with(Style::blank());

    println!("{table}");
}
