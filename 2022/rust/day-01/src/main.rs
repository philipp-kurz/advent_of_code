use std::env;
use day_01::{Config, run};
use std::process;

/*
    Run via: `cargo run -- input.txt one`
    Part one result on input.txt:  66186
    Part two result on input.txt: 196804
 */

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
