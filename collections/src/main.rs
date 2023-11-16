pub mod hashmaps;
use std::fs::File;
use std::io::{self, Read};
use std::io::ErrorKind;

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn read_username_from_file_more_concise() -> Result<String, io::Error> {
    // This can be done more concisely by combining the file and username
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn read_username_from_file_concise() -> Result<String, io::Error> {
    // Using the ? operator is a shortcut to propagating errors
    let mut username_file = File::open("hello.txt")?;
    // The ? follows a Result value, if the value is T we return from this expression
    // if the value is Err, we return the Error TO THE CALLER
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    // The ? also converts the returned error to the type defined in the return type
    Ok(username)
}
fn read_username_from_file() -> Result<String, io::Error> {
    // This returns the username OR an error to be dealt with by the caller
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn main() {
    // Results
    let greeting_file_result = File::open("hello.txt");
    // Generic type T in Result is now fs::File, E is now io:Error

    // Now handle the result with match
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        // More basic error handling
        // Err(error) => panic!("Problem opening the file: {:?}", error),
        Err(error) => match error.kind() {
            // Try to create the file if it's not there
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
        // This same logic can be accomplished more concisely with closures
    };

    // This is a string literal, no concat
    let s = "Sup nerds";
    // This makes it a String
    let S = s.to_string();
    let add = "!".to_string();
    // Add strings together w/ ownership
    // Has to be mutable to push string slice on later
    // This uses deref coercion under the hood to turn S (String) into &str
    // ALSO, this takes ownership of S, but NOT add
    let mut msg = S+&add;
    println!("{msg}");

    // Using the format macro is better for big concats
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let final_s = format!("{s1}-{s2}-{s3}");

    // Or use string slices w/o ownership
    msg.push_str(" How's it livin'?");
    println!("{msg}");

    // NOTE: Strings are utf-8 encoded, meaning one "letter" is 1-4 bytes depending
    //       so indexing might work weird if you're using non-ASCII stuff
    //       These "letters" are grapheme clusters, not chars that Rust understands

    // You can do things with string slices, but you have to be careful, because
    // each grapheme cluster here is 2 bytes so a 0..1 slice will panic
    // This doesn't operate on chars! It operates on slices! No protection!
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    // Grapheme cluster operations can be found in crates, not the std lib

    // These are the char and bytes std lib operations
    for c in "Зд".chars() {
        println!("{c}");
    }
    for b in "Зд".bytes() {
        println!("{b}");
    }

    // Vectors can only hold one type, so we can use Enums to have different things
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // Do some hashmap stuff
    hashmaps::do_a_thing();
}
