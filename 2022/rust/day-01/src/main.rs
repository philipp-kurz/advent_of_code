use aoc2022_01_rust as aoc;
use std::env;
use aoc::Config;
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

    if let Err(e) = aoc::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
