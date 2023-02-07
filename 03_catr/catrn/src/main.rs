fn main() {
    println!("Hello, world!");

    if let Err(e) = catrn::get_args().and_then(catrn::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
