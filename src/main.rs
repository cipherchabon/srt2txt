fn main() {
    if let Err(e) = srt2txt::get_args().and_then(srt2txt::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
