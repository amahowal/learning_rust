use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    // If we use a reference counter for the parent, we'd create a reference cylce between the
    // children and parent. SO use weak reference. Logically, parent should own the children and
    // children should weakly refer to the parent
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    // To be able to print the weak reference, we must borrow and upgrade
    //println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            // But here we CLONE the child because we want to own it
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        // We deref and borrow mutably the leaf parent so that we can set the
        // parent field to the downgraded (weak) reference to branch
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        // Now it will have 1 strong and 1 weak
        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        // Leave will have 2 strong, because branch has a clone
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    // Now branch goes out of scope and strong count goes to 0 and branch is cleaned up even though
    // there was still a weak reference in leaf
    }

    // Leaf won't have a parent since branch was dropped
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
