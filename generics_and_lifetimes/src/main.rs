use generics_and_lifetimes::{Summary, Tweet};

// Good performance of generics is achieved by monomorphism
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

// Different types used for fn and for struct
impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// We can impl a function for the generic type
// impl Point<T> {

// This method is declared ONLY on the concrete type f32 substitute for the generic
//impl Point<f32> {
//    fn distance_from_origin(&self) -> f32 {
//        (self.x.powi(2) + self.y.powi(2)).sqrt()
//    }
//}

// This is the std lib option enum
enum Option<T> {
    Some(T),
    None,
}

fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        // This won't work unless we restrict possible types T to ones that can do >
        //if item > largest {
        //    largest = item;
        //}
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    // This won't work because the first value sets the type to integer
    // let wont_work = Point { x: 5, y: 4.0 };
    // To make this work you could use multiple generic params

    // Showing off some different types for functions and for the struct def
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    // We can only call the trait method when we bring the trait Summary AND the type Tweet into scope
    println!("1 new tweet: {}", tweet.summarize());

    // Playing with trait lifetimes
    let string1 = String::from("abcd");
    let string2 = "xyz";

    // These are string slices (which are references)
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// We must annotate the lifetimes
// When all the lifetimes are annotated with the generic 'a this means that all reference
// parameters must have a lifetime of at least 'a
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // The compiler doesn't know whether we're returning a reference to x or y and doesn't know
    // the lifetime so it won't compile!
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// &i32 this is a reference
// &'a i32 this is an explicit lifetime annotation on a reference
// &'a mut i32 this is an explicit lifetime annotation on a mutable reference
