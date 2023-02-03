use std::fmt::Error;

use clap::{App, Arg};

type MyResult<T> = Result<T, Box<Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.0.1")
        .author("saber")
        .about("catr demo")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .default_value("-")
                .min_values(1),
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
                .help("Number non-blank lines")
                .short("b")
                .long("number-nonblank")
                .takes_value(false),
        )
        .get_matches();
    let files = matches.values_of_lossy("files").unwrap();
    Ok(Config {
        files,
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_nonblank"),
    })
}
pub fn run(config: Config) -> MyResult<()> {
    // dbg!(config);
    Ok(())
}
