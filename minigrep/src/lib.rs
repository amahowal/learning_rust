use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
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

        Ok(Config { query, file_path })
    }
}

// Box is a trait object, so the error of the Result return can be anything with Error
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // The ? operator instead of expect will return the Error from whatever
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    // Unit type wrapped in Ok
    Ok(())
}

#[cfg(test)]
mod tests {
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
}

