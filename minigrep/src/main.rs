use std::env;
use std::process;
use std::fs;
use std::error::Error;

fn main() {
    // The first item in the arguments list will be the path to the binary
    let args: Vec<String> = env::args().collect();
    // TODO: why does debug borrow args here?
    //dbg!(args);

    // parse configuration args here
    let config = Config::build(&args).unwrap_or_else(|err| {
        // the |err| here is a closure, which is an anonymous function
        println!("ERROR: Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // run the logic
    // this error handling is similar to "unwrap_or_else" except that we return the unit type in
    // the success case so we only need to deal with an error, no need to unwrap a success returnj
    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process:exit(1);
    }
}

// Box is a trait object, so the error of the Result return can be anything with Error
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // The ? operator instead of expect will return the Error from whatever
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    // Unit type wrapped in Ok
    Ok(())
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
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
