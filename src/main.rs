use std::env;
use std::process;

use rust_grep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments {}", err);
        process::exit(1);
    });

    let filename = &config.filename;
    let query = &config.query;

    eprintln!("{}", &filename);
    eprintln!("{}", &query);
    
    if let Err(e) = rust_grep::read_file(config) {
        eprintln!("Error while reading file: {}", e);
        process::exit(1);
    }
}

