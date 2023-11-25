use std::env;

fn main() {
    // The first item in the arguments list will be the path to the binary
    let args: Vec<String> = env::args().collect();
    // TODO: why does debug borrow args here?
    //dbg!(args);
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);
}
