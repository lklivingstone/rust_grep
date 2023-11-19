use std::fs;
use std::error::Error;
use std::env;

pub fn read_file(config : Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    
    let result = if config.case_insensitive {
        search(&config.query, &contents)
    }
    else {
        search_case_insensitive(&config.query, &contents)
    };

    println!("Lines with the queries: ");
    for lines in result{
        println!("{}", lines);
    }

    Ok(())
}

pub struct Config {
    pub filename: String,
    pub query: String,
    pub case_insensitive: bool
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Arguments missing");
        }
        let filename: String = args[1].clone();
        let query: String = args[2].clone();
        let case_insensitive: bool = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            filename, query, case_insensitive
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query: &str = "ample";
        let contents: &str = "
this is a
SAMple text
sample test.
";

            assert_eq!(vec!["sample test."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query: &str = "ano";
        let contents: &str = "
here is
AnOther content
and ANOTHER line
";

        assert_eq!(vec!["AnOther content", 
                   "and ANOTHER line"], 
                   search_case_insensitive(query, contents));
    }
}

