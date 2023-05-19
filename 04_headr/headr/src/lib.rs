use std::{
    error::Error,
    format,
    fs::File,
    io::{self, BufRead, BufReader, Read},
    print, println,
};

use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;
#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches: clap::ArgMatches = App::new("headr")
        .version("0.1.0")
        .author("saber saber@qq.com")
        .about("Rust head")
        .arg(
            Arg::with_name("lines")
                .short("n")
                .long("lines")
                .value_name("LINES")
                .help("Number of lines")
                .default_value("10"),
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .value_name("BYTES")
                .help("Number of bytes")
                .takes_value(true)
                .conflicts_with("lines"),
        )
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("Input file(s)")
                .required(true)
                .default_value("-")
                .min_values(1),
        )
        .get_matches();
    let lines = matches
        .value_of("lines")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal line count -- {}", e))?;
    let bytes = matches
        .value_of("bytes")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal byte count -- {}", e))?;
    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: lines.unwrap(),
        bytes,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let num_files = config.files.len();
    for (file_num, filename) in config.files.iter().enumerate() {
        match open(&filename) {
            Err(err) => eprintln!("{} : {}", filename, err),
            Ok(mut file) => {
                if num_files > 1 {
                    println!(
                        "{}===> {} <===",
                        if file_num > 0 { "\n" } else { "" },
                        &filename
                    )
                }
                if let Some(num_bytes) = config.bytes {
                    let mut handle = file.take(num_bytes as u64);
                    let mut buffer = vec![0; num_bytes];
                    let n = handle.read(&mut buffer)?;
                    println!("{}", String::from_utf8_lossy(&buffer[..n]));

                    // let mut contents = String::new();
                    // file.read_to_string(&mut contents)?;
                    // let bytes = contents.as_bytes();
                    // println!("{}", String::from_utf8_lossy(&bytes[..num_bytes]));

                    // let bytes: Result<Vec<_>, _> = file.bytes().take(num_bytes).collect();
                    // let bytes = file.bytes().take(num_bytes).collect::<Result<Vec<_>, _>>();
                    // print!("{}", String::from_utf8_lossy(&bytes?))
                } else {
                    let mut line = String::new();
                    for _ in 0..config.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        println!("{}", line);
                        line.clear();
                    }
                }
            }
        }
    }
    Ok(())
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    // let b = 1;
    // fn a() -> i32 {
    //     b
    // }
    match val.parse::<usize>() {
        Ok(v) if v > 0 => Ok(v),
        // _ => Err(val.into()),
        // _ => Err(Into::into(val)),
        _ => Err(From::from(val)),
    }
    // val.parse::<usize>().map_err(|x| val)
    // unimplemented!()
    // Ok(val.to)
    // Ok(1)
}

#[test]
fn test_parse_positive_int() {
    let string = "hello".to_string();
    let other_string = String::from("hello");
    assert_eq!(string, other_string);
    // 测试数字字符串3
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    // 测试非数字字符串
    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    // ”0“
    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "_" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
