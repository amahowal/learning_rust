use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    println!("Input was {input}");
    //let s = "42a".to_string();
    match input.trim().parse::<i32>() {
        Ok(n) => println!("Number: {}", n),
        Err(e) => println!("Error: {}", e),
    }
}
