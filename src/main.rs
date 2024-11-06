use art_streets::{config::Config, generate_art};
use std::env;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {err}");
        std::process::exit(1)
    });

    println!("{}", generate_art(config.width, config.height));
}
