// Calculate the area of a rectangle

// Explicitly make debug available on rectangle
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// These are called "associated functions"
// If they take "self" they are called methods, not all associated functions need be methods
impl Rectangle {
    // &self is short for self: &Self
    // &mut self is used to modify the parent struct, but we almost never take ownership
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // Implement a getter
    fn width(&self) -> u32 {
        self.width
    }
    // Method with additional parameters
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // This is not a method, but IS an associated function
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let scale = 2;
    // dbg! takes and returns ownership bc not a ref
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    println!(
        "The area of the rectangle (as computed by method) is {} square pixels.",
        rect1.area()
    );

    // :? uses the debug display trait
    // this goes to STDOUT
    println!("rect1 is {:?}", rect1);

    // this goes to STDERR and doesn't take ownership
    dbg!(&rect1);

    // This makes a square
    let sq = Rectangle::square(3);
}

fn area(rectangle: &Rectangle) -> u32 {
    // immutable borrow of the instance of Rectangle
    rectangle.width * rectangle.height
}

