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
