use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    // This thread ends up stopping prematurely because the main thread that is counting to 5 gets
    // there first and closes everything out
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // spawn returns ownership of a JoinHandle that we can call join on, telling the program to
    // wait until the thread is finished before cleaning up the main thread
    // THIS IS BLOCKING
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // To use variables from the main thread in a new spawned thread we must use a closure to
    // capture those values from the environment and move ownership to the new thread
    let v = vec![1, 2, 3];

    // THIS WON'T WORK WITH AN EMPTY CLOSURE
    //let handle = thread::spawn(|| {
    // USE MOVE, we don't need to tell it to move "v", the compiler can infer it!
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();

    // Multiple Producer Single Consumer channel
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        // Send returns a result, in a robust application we'd handle this instead of unwrap
        tx.send(val).unwrap();
        // If we try to print val after we sent it on the channel, we'll get an error. This could
        // be a concurrency mistake waiting to happen!!
        //println!("val is {}", val);
    });

    // we also could use try_recv which returns a Result, useful for threads that do other stuff
    // and also check for messages every now and then
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
