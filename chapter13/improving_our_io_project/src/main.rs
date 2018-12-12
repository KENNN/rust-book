extern crate improving_our_io_project;

use std::env;
use std::process;
use improving_our_io_project::Config;
use improving_our_io_project::run;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
