use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Create a channel
    let (tx, rx) = mpsc::channel();

    // Create a second transmitter
    let tx2 = tx.clone();

    // Spawn thread 1 - sends single values
    thread::spawn(move || {
        let messages = vec!["hi", "from", "the", "thread"];
        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Spawn thread 2 - also sends messages
    thread::spawn(move || {
        let messages = vec!["more", "messages", "for", "you"];
        for msg in messages {
            tx2.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Main thread receives messages
    for received in rx {
        println!("Got: {received}");
    }
}
