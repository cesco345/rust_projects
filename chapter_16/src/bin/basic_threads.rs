use std::thread;
use std::time::Duration;

fn main() {
    // Create a vector to hold the thread handles
    let mut handles = vec![];

    // Spawn a thread that will print numbers
    let handle1 = thread::spawn(|| {
        for i in 1..=5 {
            println!("Number {i} from spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });
    handles.push(handle1);

    // Spawn another thread using move to take ownership of values
    let v = vec![1, 2, 3];
    let handle2 = thread::spawn(move || {
        println!("Vector in thread: {:?}", v);
        // v is now owned by this thread
    });
    handles.push(handle2);

    // Main thread work
    for i in 1..=3 {
        println!("Number {i} from main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
}
