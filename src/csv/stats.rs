use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use crossbeam_channel::{bounded, unbounded, Sender};
use rayon::ThreadPoolBuilder;

use crate::utils::chunk_reader::{ChunkReader, Task};
use crate::utils::column::Columns;
use crate::utils::column_stats::ColumnStats;
use crate::utils::column_type::ColumnTypes;
use crate::utils::file::{column_n, estimate_line_count_by_mb};
use crate::utils::filename::new_path;
use crate::utils::progress::Progress;

pub fn run(
    filename: &str,
    sep: &str,
    no_header: bool,
    cols: &str,
    export: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // file
    let mut path = std::env::current_dir()?;
    path.push(Path::new(filename));

    // Column
    let cols = Columns::new(cols);
    let col_type = ColumnTypes::guess_from_csv(&path, filename, sep, no_header, &cols)?;

    // open file
    let mut rdr = ChunkReader::new(&path)?;

    // header
    let name = if no_header {
        cols.artificial_n_cols(column_n(filename, sep)?)
    } else {
        let r = rdr.next()?;
        r.split(sep).map(String::from).collect()
    };

    // stats holder
    let mut stat = ColumnStats::new(&col_type, &name);
    let empty_stat = stat.clone();

    // parallel channels
    let (tx_chunk, rx_chunk) = bounded(2);
    let (tx_chunk_n_control, rx_chunk_n_control) = bounded(200);
    let (tx_result, rx_result) = unbounded();

    // progress bar
    let mut prog = Progress::new();

    // threadpool
    let pool = ThreadPoolBuilder::new().build().unwrap();

    // read
    let n = estimate_line_count_by_mb(filename, Some(5));
    pool.spawn(move || rdr.send_to_channel_in_line_chunks(tx_chunk, n));

    // parallel process
    pool.scope(|s| {
        // add chunk to threadpool for process
        s.spawn(|_| {
            for task in rx_chunk {
                tx_chunk_n_control.send(0).unwrap();

                let tx = tx_result.clone();
                let st = empty_stat.clone();
                let sep_inner = sep.to_owned();
                // println!("dispatch........");
                pool.spawn(move || parse_chunk(task, tx, st, &sep_inner));
            }

            drop(tx_result);
            drop(tx_chunk_n_control);
        });

        // receive result
        for ChunkResult { bytes, stat: o } in rx_result {
            rx_chunk_n_control.recv().unwrap();
            // println!("result-----------");
            // this is bottleneck, merge two hashset is very slow.
            stat.merge(o);

            prog.add_bytes(bytes);
            prog.add_chunks(1);
            prog.print();
        }

        prog.clear();
    });

    // refine result
    stat.finalize_stats();

    // print
    if export {
        let out = new_path(&path, "-stats");
        let mut wtr = BufWriter::new(File::create(&out)?);
        wtr.write_all(stat.to_string().as_bytes())?;
        println!("Saved to file: {}", out.display());
    } else {
        stat.print();
    }

    println!("Total rows: {}", stat.rows);
    prog.print_elapsed_time();

    Ok(())
}

struct ChunkResult {
    bytes: usize,
    stat: ColumnStats,
}

fn parse_chunk(task: Task, tx: Sender<ChunkResult>, mut stat: ColumnStats, sep: &str) {
    for l in task.lines {
        stat.parse_line(&l, sep)
    }

    tx.send(ChunkResult {
        bytes: task.bytes,
        stat,
    })
    .unwrap()
}
