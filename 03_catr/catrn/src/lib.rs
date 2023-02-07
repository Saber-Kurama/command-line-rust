use clap::Parser;
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(num_args = 1..)]
    files: Vec<String>,
    #[arg(
        short = 'n',
        long = "number",
        help = "Number lines",
        default_value_t = false,
        conflicts_with = "number_nonblank_lines"
    )]
    number_lines: bool,
    #[arg(
        short = 'b',
        long = "number-nonblank",
        help = "Number non-blank lines",
        default_value_t = false
    )]
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Args> {
    Ok(Args::parse())
}

fn open(file_name: &str) -> MyResult<Box<dyn BufRead>> {
    Ok(Box::new(BufReader::new(File::open(file_name)?)))
}

pub fn run(args: Args) -> MyResult<()> {
    println!("{:?}", args);
    for file_name in args.files {
        match open(&file_name) {
            Err(err) => eprintln!("faild filename {} : {}", file_name, err),
            Ok(file) => {
                let mut last_num = 0;
                for (line_number, line_result) in file.lines().enumerate() {
                    let line_str = line_result?;
                    if args.number_lines {
                        println!("{:>6}\t{}", line_number, line_str)
                    } else if args.number_nonblank_lines {
                        if line_str.is_empty() {
                            println!("")
                        } else {
                            last_num += 1;
                            println!("{:>6}\t{}", last_num, line_str);
                        }
                    } else {
                        println!("{}", line_str);
                    }
                }
            }
        }
    }
    Ok(())
}
