fn main() {
    if let Err(err) = headx::get_args().and_then(headx::run) {
        eprintln!("{:?}", err);
        std::process::exit(1);
    };
}
