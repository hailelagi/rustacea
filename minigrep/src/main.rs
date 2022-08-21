use std::env;
use std::process;
use minigrep::{Config};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("coudn't parse args, reason: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
