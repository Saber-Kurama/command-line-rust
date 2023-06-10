use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

pub fn get_args() -> MyResult<()> {
    return Ok(());
}

pub fn run(config: ()) -> MyResult<()> {
    return Ok(());
}
