fn main() {
    if let Err(e) = rusty_stocks::get_args().and_then(rusty_stocks::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
