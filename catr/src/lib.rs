use std::error::{self, Error};

use clap::{App, Arg};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_noblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    if config.number_lines && config.number_noblank_lines {
        return Err("the argument '--number_nonblank' cannot be used with '--number'".into());
    }
    dbg!(config);
    Ok(())
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
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .help("Number lines")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("number-nonblank")
                .short("b")
                .help("Number nonblank lines")
                .takes_value(false),
        )
        .get_matches();
    Ok(Config {
        files: matches.values_of_lossy("file").unwrap(),
        number_lines: if matches.is_present("number") {
            true
        } else {
            false
        },
        number_noblank_lines: if matches.is_present("number-nonblank") {
            true
        } else {
            false
        },
    })
}
