use std::{env, process};
mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = lib::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = lib::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
