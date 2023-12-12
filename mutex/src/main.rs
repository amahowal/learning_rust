// Just like using RefCell to mutate contents of Rc, we can use Mutex to change contents of Arc!
// The compiler will protect against two threads using the same data at the same time, but it won't
// protect against deadlocks. Logic errors could lead to two threads waiting for each other forever
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // unfortunately, Rc is not threadsafe, so we use Arc instead
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        //let counter = Rc::clone(&counter);
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // lock returns a MutexGuard in a LockResult
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        // when the guard goes out of scope it has a Drop implementation that releases the lock
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
