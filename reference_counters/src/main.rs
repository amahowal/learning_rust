// KEY CONCEPT: Interior mutability, which allows you to mutate data even when there are immutable
// references to it. This is disallowed by the compiler, but we use "unsafe" code to accomplish
// this and wrap it in a "safe" API for the user.

enum List {
    // Box won't work because more than one owns it
    // could use lifetimes, but won't always be certain about them
    //Cons(i32, Box<List>),
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    //let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    //let b = Cons(3, Box::new(a));
    //let c = Cons(4, Box::new(a));

    // Creating new references and then using Rc::clone to share the reference and count it
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    // This is great!!! But also immutable or else we could modify data without other parts of our
    // logic knowing about it! Enter: RefCell<T>

    // With RefCell<T> the borrowing rules invariants are enforced AT RUNTIME
    //
}
