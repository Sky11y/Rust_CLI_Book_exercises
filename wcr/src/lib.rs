use clap::{App, Arg};
use count_digits::CountDigits;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wcr")
        .version("0.1.0")
        .author("Sky11y <sky11y1991@gmail.com")
        .about("Rust wc")
        .arg(
            Arg::with_name("files")
                .help("Input file(s)")
                .value_name("FILE")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("lines")
                .help("Display line count")
                .short("l")
                .long("lines")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("words")
                .help("Display word count")
                .short("w")
                .long("words")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("bytes")
                .help("Display byte count")
                .short("c")
                .long("bytes")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("chars")
                .help("Display char count")
                .short("m")
                .long("chars")
                .takes_value(false)
                .conflicts_with("bytes"),
        )
        .get_matches();

    let mut lines = matches.is_present("lines");
    let mut words = matches.is_present("words");
    let mut bytes = matches.is_present("bytes");
    let chars = matches.is_present("chars");

    if [lines, words, bytes, chars].iter().all(|v| v == &false) {
        lines = true;
        words = true;
        bytes = true;
    }

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines,
        words,
        bytes,
        chars,
    })
}

pub enum ReturnValue {
    Success(String, FileInfo),
    Fail(String),
}

pub fn run(config: Config) -> MyResult<()> {
    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;
    let mut total_chars = 0;

    let mut results: Vec<ReturnValue> = Vec::new();

    for filename in &config.files {
        match open(&filename) {
            Err(e) => {
                let error = ReturnValue::Fail(format!("{}: {}", filename, e));
                results.push(error);
            }
            Ok(file) => {
                if let Ok(info) = count(file) {
                    total_lines += info.num_lines;
                    total_words += info.num_words;
                    total_bytes += info.num_bytes;
                    total_chars += info.num_chars;

                    results.push(ReturnValue::Success(filename.to_owned(), info));
                }
            }
        }
    }

    let value_count: usize = [config.lines, config.words, config.bytes, config.chars]
        .iter()
        .filter(|&n| *n == true)
        .count();

    /* gcc version of wc has different paddings depending of input */
    let padding = if config.files[0] == "-" {
        7
    } else if value_count > 1 || config.files.len() > 1 {
        total_bytes.count_digits()
    } else {
        0
    };

    for res in results {
        match res {
            ReturnValue::Fail(s) => eprintln!("{}", s),
            ReturnValue::Success(filename, info) => {
                let mut values_to_print = value_count;
                println!(
                    "{}{}{}{}{}",
                    format_field(info.num_lines, config.lines, padding, &mut values_to_print),
                    format_field(info.num_words, config.words, padding, &mut values_to_print),
                    format_field(info.num_bytes, config.bytes, padding, &mut values_to_print),
                    format_field(info.num_chars, config.chars, padding, &mut values_to_print),
                    if filename == "-" {
                        "".to_string()
                    } else {
                        format!(" {}", filename)
                    }
                );
            }
        }
    }

    if config.files.len() > 1 {
        let mut values_to_print = value_count;
        println!(
            "{}{}{}{} total",
            format_field(total_lines, config.lines, padding, &mut values_to_print),
            format_field(total_words, config.words, padding, &mut values_to_print),
            format_field(total_bytes, config.bytes, padding, &mut values_to_print),
            format_field(total_chars, config.chars, padding, &mut values_to_print),
        );
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn format_field(value: usize, show: bool, padding: usize, values_to_print: &mut usize) -> String {
    if show {
        let add_space = if *values_to_print > 1 {
            *values_to_print -= 1;
            " ".to_string()
        } else {
            "".to_string()
        };
        format!("{:>width$}{}", value, add_space, width = padding)
    } else {
        "".to_string()
    }
}

fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;
    let mut line = String::new();

    loop {
        let line_bytes = file.read_line(&mut line)?;
        if line_bytes == 0 {
            break;
        }
        num_bytes += line_bytes;
        num_lines += 1;
        num_words += line.split_whitespace().count();
        num_chars += line.chars().count();
        line.clear();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

#[cfg(test)]
mod tests {
    use super::{count, format_field, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }

    #[test]
    fn test_format_field() {
        assert_eq!(format_field(1, false, 0, &mut 0), "");
        assert_eq!(format_field(3, true, 2, &mut 0), " 3");
        assert_eq!(format_field(10, true, 3, &mut 2), " 10 ");
    }
}
