use std::io;
// use rand::Rng;

fn main() {
    // The nth Fibonacci number is the sum of the (n-1)th and the (n-2)th number
    println!("Input the Fibonacci index you would like to generate!");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let index:i32 = input.trim().parse().expect("Not a valid number");
    //match input.trim().parse::<i32>() {
    //    Ok(n) => println!("Number: {}", n),
    //    Err(e) => println!("Error: {}", e),
    //};

    // TODO: handle 0 and 1 cases
    let result = calculate_fibonacci(index);
    println!("The Fibonacci number for index {index} is: {result}");

    // let secret_number = rand::thread_rng().gen_range(1..=100);
}

fn calculate_fibonacci(index: i32) -> i32 {
    // first two numbers 0, 1
    let mut a = 0;
    let mut b = 1;
    let mut result = 0;
    for _i in 2..index {
        result = a + b;
        a = b;
        b = result;
    }
    println!("The next number is: {b}");
    return result;
}
