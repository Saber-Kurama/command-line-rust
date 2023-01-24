use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // #[arg(short, long)]
    // text: String,
    /// Name of the person to greet
    #[arg(num_args = 1..)]
    text: Vec<String>,
    #[arg(short = 'n', long = "noline", default_value_t = false)]
    omit_newline: bool,
}

fn main() {
    let args = Args::parse();
    print!(
        "{}{}",
        args.text.join(" "),
        if args.omit_newline { "" } else { "\n" }
    );
}
