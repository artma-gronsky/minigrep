use std::{error::Error, fs, env};

pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
    pub ignore_case: bool
}

impl Config<'_> {
    pub fn build(args: &[String]) -> Result<Config, &str> {
        Self::validate_args(args)?;

        let query = &args[1];
        let file_path = &args[2];

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }

    fn validate_args(args: &[String]) -> Result<(), &str> {
        if args.len() < 3 {
            return Err("Wrong arguments length");
        }

        Ok(())
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(config.query, &contents)
    }else{
        search(config.query, &contents)
    };

    for line in results{
        println!("{line}");
    }

    Ok(())
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            result.push(line.trim())
        }
    }

    result
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str>{
    let lowercase_query = query.to_lowercase();
    content.lines().filter(|l| l.to_lowercase().contains(&lowercase_query)).collect()   
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "produ";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        let expect = vec!["safe, fast, productive."];
        let result = search(query, contents);

        assert_eq!(expect, result);
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
); }
}
