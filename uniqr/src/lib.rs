use clap::Parser;
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
    let mut infile = open(&args.infile).map_err(|e| format!("{}: {}", args.infile, e))?;

    let mut outfile: Box<dyn Write> = match &args.outfile {
        Some(outfile) => Box::new(File::create(outfile)?),
        _ => Box::new(io::stdout()),
    };
    let mut line = String::new();
    let mut previous = String::new();
    let mut count: u64 = 0;

    let mut print = |count: u64, text: &str| -> MyResult<()> {
        if count > 0 {
            if args.count {
                write!(outfile, "{:>7} {}", count, text)?;
            } else {
                write!(outfile, "{}", text)?;
            }
        };
        Ok(())
    };

    loop {
        let bytes = infile.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }

        if line.trim_end() != previous.trim_end() {
            print(count, &previous)?;
            previous = line.clone();
            count = 0;
        }

        count += 1;
        line.clear();
    }

    // if file is not empty add newline to the end of file if it doesn't have one.
    if previous.len() > 0 && previous.chars().last().unwrap() != '\n' {
        previous.push('\n');
    }
    print(count, &previous)?;
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
