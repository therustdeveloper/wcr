fn main() {
    if let Err(e) = wcr::exec() {
        eprintln!("{}", e);
        std::process::exit(1)
    }
}
