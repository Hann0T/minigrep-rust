use minigrep_2::{run, Config};
use std::{env, process};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Query, Searching for: {}", config.query);
    println!("In File: {}", config.file_name);

    if let Err(e) = run(config) {
        eprintln!("Application Error, {}", e);
        process::exit(1);
    }
}
