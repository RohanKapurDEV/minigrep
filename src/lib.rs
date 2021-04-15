use std::{env, error::Error, fs};

pub fn run(conf: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(conf.filename)?;

    let results = if conf.case_sensitive {
        search(&conf.query, &contents)
    } else {
        search_case_insensitive(&conf.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

fn search<'a, 'b>(query: &'a str, content: &'b str) -> Vec<&'b str> {
    let mut results: Vec<&str> = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    if results.len() == 0 {
        println!("No match found :(");
    }

    results
}

fn search_case_insensitive<'a, 'b>(query: &'a str, content: &'b str) -> Vec<&'b str> {
    let query = query.to_lowercase();
    let mut results: Vec<&str> = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line)
        }
    }

    results
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("You need to pass in 2 arugments to Minigrep.");
        } else if args.len() > 3 {
            return Err("You can only pass in 2 arugments to Minigrep, not more.");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}
