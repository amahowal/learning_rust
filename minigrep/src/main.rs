use std::env;
use std::process;

use minigrep::Config;

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
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

