use std::env;

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    if let Err(err) = safe_lang::cli::run(args) {
        eprintln!("{err}");
        std::process::exit(1);
    }
}
