use std::thread;
use std::time::Duration;
// closures and functions share the way they take parameters
// 1. borrow mutably
// 2. borrow immutably
// 3. take ownership
// functions take parameters this way while closures take these from their environment

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    // This is going to use FnMut as the trait bound because it doesn't move anything out of the
    // environment in which it is defined (or capture or mutate)
    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);
    // Whereas FnOnce is always a trait bound, but when it is the ONLY trait bound we cannot call
    // the closure more than once

    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    // we can also define closures and assign them to variables
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    // This closure infers that it only needs an immutable reference so that's all it does
    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling immutable ref closure: {:?}", list);
    only_borrows();
    println!("After calling immutable ref closure: {:?}", list);

    // This closure infers that it needs a mutable reference because it has to add to the vector
    let mut borrows_mutably = || list.push(7);

    // CAN'T DO THIS!! because the "borrows_mutably" definition captures the mutalbe reference and
    // the compiler uses it later, we can't immutably borrow before borrows_mutably is called for
    // the last time
    //println!("Before calling mutable ref closure: {:?}", list);
    borrows_mutably();
    println!("After calling mutable ref closure: {:?}", list);

    // Lastly, moving ownership even if it's not infered is possible. This is useful when spawning
    // threads
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
    println!("End of main");
}

// NOTE: functions and closures are very similar, but differ in the syntax and how much inference
// the compiler can do wrt to types
// fn  add_one_v1   (x: u32) -> u32 { x + 1 }
// let add_one_v4 = |x|               x + 1  ;
// THESE ARE THE SAME!!
// The compiler infers on closures from usage, so v4 has no type until we use it, and then it can
// only be used for that type!
