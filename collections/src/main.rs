enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
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
}
