use std::{error::Error, fs, env};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query_options = args.next();
        let file_path_options = args.next();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Self::validate_args(&query_options, &file_path_options)?;
        let query = query_options.unwrap();
        let file_path = file_path_options.unwrap();
        

        Ok(Config { query, file_path, ignore_case })
    }

    fn validate_args(query: &Option<String>, file_path: &Option<String>) -> Result<(), &'static str> {
        if query.is_none() {
            return Err("Didn't get a query string")
        }

        if file_path.is_none(){
            return Err("Didn't get a file path")
        }

        Ok(())
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    }else{
        search(&config.query, &contents)
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
