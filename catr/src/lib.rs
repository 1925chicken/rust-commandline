use std::{
    error::Error,
    fs::{read, File},
    io::{self, BufRead, BufReader},
};

use clap::{App, Arg};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_noblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => readlines(config.number_lines, config.number_noblank_lines, file),
        }
    }
    Ok(())
}

fn readlines(number_lines: bool, number_noblank_lines: bool, file: Box<dyn BufRead>) {
    let reader = BufReader::new(file);
    if number_lines {
        readlines_with_number(reader);
        return;
    }

    if number_noblank_lines {
        readlines_with_number_no_blank_line(reader);
        return;
    }

    readlines_no_option(reader);
}

fn readlines_no_option(reader: BufReader<Box<dyn BufRead>>) {
    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}

fn readlines_with_number(reader: BufReader<Box<dyn BufRead>>) {
    for (line_num, line) in reader.lines().enumerate() {
        println!("{:>6}\t{}", line_num + 1, line.unwrap());
    }
}

fn readlines_with_number_no_blank_line(reader: BufReader<Box<dyn BufRead>>) {
    let mut cnt = 0;
    for line in reader.lines() {
        let l = line.unwrap();
        if l.is_empty() {
            println!();
            continue;
        }
        cnt += 1;
        println!("{:>6}\t{}", cnt, l);
    }
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("hoge hoge")
        .about("Rust cat")
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .help("Input file(s)")
                .required(true)
                .default_value("-")
                .multiple(true),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .help("Number lines")
                .takes_value(false)
                .conflicts_with("number_nonblank"),
        )
        .arg(
            Arg::with_name("number_nonblank")
                .short("b")
                .long("number-nonblank")
                .help("Number non-blank lines")
                .takes_value(false),
        )
        .get_matches();
    Ok(Config {
        files: matches.values_of_lossy("file").unwrap(),
        number_lines: matches.is_present("number"),
        number_noblank_lines: matches.is_present("number_nonblank"),
    })
}
