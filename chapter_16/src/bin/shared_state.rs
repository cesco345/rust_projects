use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Create a mutex containing a counter
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // Spawn 10 threads that will each increment the counter
    for _ in 0..10 {
        // Clone the Arc to get another reference to the mutex
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            // Lock the mutex to get access to the counter
            let mut num = counter.lock().unwrap();
            *num += 1;
            // Lock is automatically released here when num goes out of scope
        });

        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Print the final value
    println!("Final count: {}", *counter.lock().unwrap());
}
