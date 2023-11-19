use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments {}", err);
        process::exit(1);
    });

    let filename = &config.filename;
    let query = &config.query;

    println!("{}", &filename);
    println!("{}", &query);
    
    if let Err(e) = read_file(config) {
        println!("Error while reading file: {}", e);
        process::exit(1);
    }
}

fn read_file(config : Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;

    println!("{}", &content);
    Ok(())
}

struct Config {
    filename: String,
    query: String
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Arguments missing");
        }
        let filename: String = args[1].clone();
        let query: String = args[2].clone();

        Ok(Config {
            filename, query
        })
    }
}
