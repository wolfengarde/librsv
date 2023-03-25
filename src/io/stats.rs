use crate::utils::cli_result::CliResult;
use crate::utils::column::Columns;
use crate::utils::column_stats::ColumnStats;
use crate::utils::column_type::ColumnTypes;
use crate::utils::filename::new_file;
use crate::utils::reader::IoReader;
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufWriter, Write};

pub fn run(sep: &str, no_header: bool, cols: &str, export: bool) -> CliResult {
    // read
    let rows = IoReader::new().lines();

    // too few rows
    if rows.len() <= 1 - no_header as usize {
        return Ok(());
    }

    // split rows
    let n = rows[0].split(sep).count();
    let cols = Columns::new(cols).total_col(n).parse();
    let rows = rows
        .par_iter()
        .map(|r| r.split(sep).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // header
    let names = match no_header {
        true => cols.artificial_n_cols(rows[0].len()),
        false => rows[0].iter().map(|&i| i.to_owned()).collect::<Vec<_>>(),
    };

    let rows = &rows[(1 - no_header as usize)..];

    // column type
    let typ = ColumnTypes::guess_from_io(rows, &cols);

    // stats holder
    let mut stat = ColumnStats::new(&typ, &names);
    let chunks = rows.chunks(1000).collect::<Vec<_>>();
    let r = chunks
        .into_par_iter()
        .map(|chunk| {
            let mut s = stat.clone();
            for r in chunk {
                s.parse_line_by_fields(r);
            }
            s
        })
        .collect::<Vec<_>>();
    r.into_iter().fold(&mut stat, |s, b| {
        s.merge(b);
        s
    });

    stat.cal_unique_and_mean();

    if export {
        let out = new_file("stats.csv");
        let mut wtr = BufWriter::new(File::create(&out)?);
        wtr.write_all(stat.to_string().as_bytes())?;
        println!("Saved to file: {}", out.display());
    } else {
        stat.print();
        println!("Total rows: {}", stat.rows);
    }

    Ok(())
}
