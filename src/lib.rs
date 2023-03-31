use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub options: Vec<String>,
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {

        args.next();

        let col_args: Vec<String> = args.collect();

        let mut options: Vec<String> = Vec::new();
        let mut query: String = String::new();
        let mut file_path: String = String::new();

        for arg in col_args {
            if arg.starts_with('-') {
                options.push(arg[1..].to_string());

            } else if arg.ends_with(".txt") {
                file_path = arg;

            } else {
                query = arg;
            }
        }

        if query.is_empty() {
            return Err("Didn't get a query string");
        }

        if file_path.is_empty() {
            return Err("Didn't get a file path");
        }

        Ok(Config { 
            query, 
            file_path,
            options,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    
    let results = if config.options.contains(&"ci".to_string()) {
        println!("AAA");
        search_case_insensitive(&config.query, &contents)
    } else {
        println!("BBB");
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(
    query: &str, 
    contents: &'a str,
) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        )
    }
}