use std::error::Error;

use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;
#[derive(Debug)]
pub struct Config {
    // files: Vec<String>,
    // lines: usize,
    // bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("saber")
        .about("Rust head")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("Input Text")
                // .required(true)
                .min_values(1),
        )
        .get_matches();
    Ok(Config {})
}

pub fn run(options: Config) -> MyResult<()> {
    println!("{:#?}", options);
    Ok(())
}
