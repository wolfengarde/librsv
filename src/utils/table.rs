use super::{cli_result::CliResult, constants::TERMINATOR};
use std::{
    borrow::Cow,
    io::{stdout, BufWriter, Write},
};
use tabled::{builder::Builder, Style};

pub struct Table<'a> {
    builder: Builder<'a>,
    n: usize,
}

impl<'a> Table<'a> {
    #[allow(dead_code)]
    fn new() -> Self {
        Table {
            builder: Builder::default(),
            n: 0,
        }
    }

    fn is_empty(&self) -> bool {
        self.n == 0
    }

    #[allow(dead_code)]
    fn add_record<R, T>(&mut self, row: R)
    where
        R: IntoIterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.builder.add_record(row);
        self.n += 1;
    }

    #[allow(dead_code)]
    fn add_records<R, T>(&mut self, rows: Vec<R>)
    where
        R: IntoIterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.n += rows.len();
        for row in rows {
            self.builder.add_record(row);
        }
    }

    pub fn from_rows(rows: &'a Vec<String>, sep: &str) -> Self {
        let r = rows
            .iter()
            .map(|i| i.split(sep).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut b = Builder::default();
        let n = rows.len();
        for row in r {
            b.add_record(row);
        }

        Table { builder: b, n }
    }

    pub fn from_records<R, T>(rows: Vec<R>) -> Self
    where
        R: IntoIterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        let mut b = Builder::default();
        let n = rows.len();

        for row in rows {
            b.add_record(row);
        }

        Table { builder: b, n }
    }

    pub fn print_blank(self) -> CliResult {
        if self.is_empty() {
            return Ok(());
        }

        // build
        let mut table = self.builder.build();
        table.with(Style::blank());

        // print
        let mut wtr = BufWriter::new(stdout());
        wtr.write_all(table.to_string().as_bytes())?;
        wtr.write_all(TERMINATOR)?;

        Ok(())
    }
}
