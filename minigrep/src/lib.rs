use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // magic number 3 because we have 3 arguments (first is exe pointer)
        if args.len() < 3 {
            // panic here is not reallyl the best thing to do, better to return Result
            //panic!("Must specify all arguments (query, file_path)");
            return Err("Must specify all arguments (query, file_path)");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        // is_ok is a method on Result type that returns a bool
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

// Box is a trait object, so the error of the Result return can be anything with Error
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // The ? operator instead of expect will return the Error from whatever
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{contents}");

    let results = if config.ignore_case {
        case_insensitive_search(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    // Unit type wrapped in Ok
    Ok(())
}

// lifetime on contents argument and return value a
// we are returning a vector of references to slices of contents so the return must live at least
// as long as contents
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn case_insensitive_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    // this is a shadowed variable of the same name
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        // This backslash tells the compiler not to prepend a newline char
        let contents = "\
Rust:
safe, fast, productive.
Duct tape.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "Duct";
        // This backslash tells the compiler not to prepend a newline char
        let contents = "\
Duct tape:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["Duct tape:", "safe, fast, productive."], case_insensitive_search(query, contents));
    }
}

