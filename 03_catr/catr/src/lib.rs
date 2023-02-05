use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

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
                .value_name("XXXX")
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

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    // dbg!(config);
    for filename in config.files {
        println!("filename: {}", filename);
        match open(&filename) {
            Err(err) => eprintln!("faild filename {} : {}", filename, err),
            Ok(file) => {
                for line_result in file.lines() {
                    let line = line_result?;
                    println!("{}", line)
                }
            }
        }
    }
    Ok(())
}
