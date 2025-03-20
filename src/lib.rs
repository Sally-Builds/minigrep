use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;


pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Arguments not complete");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();

    f.read_to_string(&mut contents)?;

    let result = if config.case_sensitive {
        search(&config.query, &contents)
    }else {
        search_case_insensitive(&config.query, &contents)
    };

    for res in result {
        println!("{res}");
    }


    Ok(())
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = vec![];
    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = vec![];
    let query = query.to_lowercase();
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}


#[cfg(test)]
mod tests {
    use crate::*;

    
    #[test]
    fn should_create_a_new_config() {
        let args = vec![String::from("target/debug/minigrep"),String::from("hello"), String::from("test.txt")];

        let config = Config::new(&args).unwrap();

        assert_eq!(&args[1], &config.query);
        assert_eq!(&args[2], &config.filename);
    }

    #[test]
    fn should_return_error_if_arguments_is_less_than_three() {
        let args = vec![String::from("target/debug/minigrep")];

        if let Err(e) = Config::new(&args) {
            assert_eq!(e, "Arguments not complete");
        }

    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive,
Pick  three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive,"], search(query, content));
    }

    #[test]
    fn case_insensitive() {
        let query = "RuSt";
        let content = "\
Rust:
safe, fast, productive,
Pick  three.
Trust me";

        assert_eq!(vec!["Rust:", "Trust me"], search_case_insensitive(query, content));
    }
}