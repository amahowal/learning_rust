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
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// This is the std lib option enum
enum Option<T> {
    Some(T),
    None,
}

fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        // This won't work unless we restrict possible types T to ones that can do >
        if item > largest {
            largest = item;
        }
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
}
