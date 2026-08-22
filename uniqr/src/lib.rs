use clap::Parser;
#[allow(unused_imports)]
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Write},
};

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `uniq`
pub struct Opts {
    /// Input file
    #[arg(value_name = "INFILE", default_value = "-")]
    infile: String,

    /// Output file
    #[arg(value_name = "OUTFILE")]
    outfile: Option<String>,

    /// Show counts
    #[arg(short, long)]
    count: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(args: Opts) -> MyResult<()> {
    println!("{:#?}", args);
    Ok(())
}
