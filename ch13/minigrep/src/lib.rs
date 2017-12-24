use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub struct Config {
    pub command: String,
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        let command = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a command - which shouldn't happen"),
        };
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { command, query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(&config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = search(&config, &contents);

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(config: &Config, contents: &'a str) -> Vec<&'a str> {
    // TODO: Could this be optimized further?
    let lines = contents.lines();
    if config.case_sensitive {
        return lines
            .filter(|line| line.contains(&config.query))
            .collect();
    } else {
        let query_lc = config.query.to_lowercase();
        return lines
            .filter(|line| line.to_lowercase().contains(&query_lc))
            .collect();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let config = Config {
            command: String::from(""),
            query: String::from("duct"),
            filename: String::from("foo.txt"),
            case_sensitive: true,
        };
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(&config, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let config = Config {
            command: String::from(""),
            query: String::from("rUsT"),
            filename: String::from("foo.txt"),
            case_sensitive: false,
        };
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search(&config, contents)
        );
    }
}
