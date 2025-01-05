use std::env;
use std::process;

use lc3;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = lc3::Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(err) = lc3::run(config) {
        println!("Application error: {err}");
        process::exit(1);
    }
}
