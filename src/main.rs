extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;
use minigrep::run;

fn main() {
    // let args: Vec<String> = env::args().collect();
    let config = Config::new(env::args()).unwrap_or_else(|e| {
        eprintln!("[ARG]: {}", e);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("[APP]: {}", e);
        process::exit(1);
    }
}
