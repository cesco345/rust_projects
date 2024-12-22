use std::sync::{Arc, Mutex};
use std::thread;

// A custom type that implements Send and Sync
#[derive(Debug)]
struct Counter {
    count: u32,
}

// Safely share Counter between threads by wrapping in Arc and Mutex
fn share_counter() {
    let counter = Arc::new(Mutex::new(Counter { count: 0 }));
    let mut handles = vec![];

    for _ in 0..3 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut counter = counter.lock().unwrap();
            counter.count += 1;
            println!("Counter in thread: {:?}", counter);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter: {:?}", *counter.lock().unwrap());
}

// Example of a type that is not Send
use std::rc::Rc;
fn demonstrate_not_send() {
    let not_send = Rc::new(42);

    // This would not compile:
    // thread::spawn(move || {
    //     println!("Rc in thread: {}", *not_send);
    // });
}

fn main() {
    share_counter();
    demonstrate_not_send();
}
