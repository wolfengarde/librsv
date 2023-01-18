use crate::utils::cli_result::CliResult;
use crate::utils::excel_reader::ExcelReader;
use crate::utils::filename::new_path;
use crate::utils::util::werr;
use crate::utils::writer::Writer;
use std::path::Path;
use std::process;

#[allow(clippy::too_many_arguments)]
pub fn run(
    path: &Path,
    sheet: usize,
    no_header: bool,
    start: usize,
    end: Option<usize>,
    length: Option<usize>,
    index: Option<usize>,
    export: bool,
) -> CliResult {
    // out file
    let out_path = new_path(path, "-slice");

    // open file
    let mut wtr = Writer::file_or_stdout(export, &out_path)?;
    let mut rdr = ExcelReader::new(path, sheet)?;

    // header
    if !no_header {
        match rdr.next() {
            Some(v) => wtr.write_excel_line_unchecked(v),
            None => return Ok(()),
        }
    }

    // slice
    match index {
        Some(index) => write_by_index(&mut rdr, &mut wtr, index),
        None => {
            let end = end
                .or_else(|| length.map(|l| start + l).or(Some(usize::MAX - 10)))
                .unwrap();
            if start > end {
                werr!("Error: end index should be equal to or bigger than start index.");
                process::exit(1)
            }
            write_by_range(&mut rdr, &mut wtr, start, end);
        }
    }

    if export {
        println!("Saved to file: {}", out_path.display())
    }

    Ok(())
}

fn write_by_index(rdr: &mut ExcelReader, wtr: &mut Writer, index: usize) {
    for r in rdr.iter().skip(rdr.next_called + index).take(1) {
        wtr.write_excel_line_unchecked(r);
    }
}

fn write_by_range(rdr: &mut ExcelReader, wtr: &mut Writer, start: usize, end: usize) {
    for r in rdr.iter().skip(rdr.next_called + start).take(end - start) {
        wtr.write_excel_line_unchecked(r);
    }
}
