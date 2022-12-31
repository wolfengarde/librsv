use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn estimate(filename: &str) -> Result<i32, Box<dyn std::error::Error>> {
    // current file
    let mut path = std::env::current_dir()?;
    path.push(Path::new(filename));

    // read 20000 lines to estimate bytes per line
    let file = File::open(path)?;
    let filesize = file.metadata()?.len() as f64;
    let mut total_bytes = 0;
    let mut n = 0;
    for l in BufReader::new(file).lines().skip(1) {
        total_bytes += l.unwrap().len() + 1;
        n += 1;
        if n > 20000 {
            break;
        }
    }

    // estimate line count
    let estimate_n = filesize / ((total_bytes as f64) / (n as f64));
    Ok(estimate_n as i32)
}
