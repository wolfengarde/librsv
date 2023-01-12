use crossbeam_channel::bounded;
use rayon::prelude::*;

use crate::utils::chunk_reader::{ChunkReader, Task};
use crate::utils::column::Columns;
use crate::utils::constants::TERMINATOR;
use crate::utils::file::{estimate_line_count_by_mb, file_or_stdout_wtr};
use crate::utils::filename::{full_path, new_path};
use crate::utils::filter::Filter;
use crate::utils::progress::Progress;

use std::error::Error;
use std::io::{BufWriter, Write};
use std::thread;

pub fn run(
    filename: &str,
    no_header: bool,
    sep: &str,
    cols: &str,
    filter: &str,
    export: bool,
) -> Result<(), Box<dyn Error>> {
    // current file
    let path = full_path(filename);
    let out_path = new_path(&path, "-selected");

    // filters and cols
    let filter = Filter::new(filter);
    let cols = Columns::new(cols);

    // open file
    let f = file_or_stdout_wtr(export, &out_path)?;
    let mut wtr = BufWriter::new(f);
    let mut rdr = ChunkReader::new(&path)?;

    // const
    let sep_bytes = sep.as_bytes();

    // header
    if !no_header {
        let row = rdr.next()?;
        let row = row.split(sep).collect::<Vec<_>>();
        let record = match cols.all {
            true => row,
            false => cols.iter().map(|&i| row[i]).collect(),
        };
        print_record(&mut wtr, &record, sep_bytes, TERMINATOR)?;
    }

    // parallel queue
    let (tx, rx) = bounded(1);

    // read
    let line_buffer_n: usize = estimate_line_count_by_mb(filename, None);
    thread::spawn(move || rdr.send_to_channel_in_line_chunks(tx, line_buffer_n));

    // process
    let mut prog = Progress::new();
    for task in rx {
        handle_task(
            task, &filter, sep, &cols, &mut wtr, sep_bytes, export, &mut prog,
        )
    }

    if export {
        println!("\nSaved to file: {}", out_path.display())
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn handle_task(
    task: Task,
    filter: &Filter,
    sep: &str,
    cols: &Columns,
    wtr: &mut BufWriter<Box<dyn Write>>,
    sep_bytes: &[u8],
    export: bool,
    prog: &mut Progress,
) {
    // filter
    let filtered = task
        .lines
        .par_iter()
        .filter_map(|row| filter.record_valid_map(row, sep))
        .collect::<Vec<_>>();

    // write
    filtered.iter().for_each(|row| {
        if cols.all {
            print_record(wtr, row, sep_bytes, TERMINATOR).unwrap()
        } else {
            let record = cols.iter().map(|&i| row[i]).collect::<Vec<_>>();
            print_record(wtr, &record, sep_bytes, TERMINATOR).unwrap()
        }
    });

    if export {
        prog.add_chunks(1);
        prog.add_bytes(task.bytes);
        prog.print();
    }
}

fn print_record(
    wtr: &mut BufWriter<Box<dyn Write>>,
    record: &[&str],
    sep_bytes: &[u8],
    terminator: &[u8],
) -> std::io::Result<()> {
    let mut it = record.iter().peekable();

    while let Some(&field) = it.next() {
        wtr.write_all(field.as_bytes())?;

        if it.peek().is_none() {
            wtr.write_all(terminator)?;
        } else {
            wtr.write_all(sep_bytes)?;
        }
    }

    Ok(())
}