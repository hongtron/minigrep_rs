use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let raw_args: Box<Vec<String>> = Box::new(env::args().collect());
    // let args = Box::new(raw_args);
    let config = Config::new(raw_args).unwrap_or_else(|err| {
        eprintln!("Problem parsing argument: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}

