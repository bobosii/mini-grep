use std::process;
use std::env;
use mini_grep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem with parsing elements {}", err);
        process::exit(1);
    });

    println!("Searching for query {}", config.query);
    println!("In this file {}", config.filename);

    if let Err(e) = mini_grep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
