use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

struct WorkItem {
    id: u32,
    data: String,
}

struct WorkResult {
    id: u32,
    result: String,
}

fn main() {
    // Channel for sending work results
    let (tx, rx) = mpsc::channel();

    // Shared queue of work items
    let work_queue = Arc::new(Mutex::new(Vec::new()));

    // Add some work items to the queue
    {
        let mut queue = work_queue.lock().unwrap();
        for i in 1..=5 {
            queue.push(WorkItem {
                id: i,
                data: format!("Task {}", i),
            });
        }
    }

    // Create worker threads
    let mut handles = vec![];
    for worker_id in 1..=3 {
        let queue = Arc::clone(&work_queue);
        let tx = tx.clone();

        let handle = thread::spawn(move || {
            loop {
                // Try to get a work item
                let work_item = {
                    let mut queue = queue.lock().unwrap();
                    queue.pop()
                };

                match work_item {
                    Some(item) => {
                        // Process the work item
                        println!("Worker {} processing task {}", worker_id, item.id);
                        thread::sleep(Duration::from_millis(500));

                        // Send the result back
                        tx.send(WorkResult {
                            id: item.id,
                            result: format!("Result of task {}", item.id),
                        })
                        .unwrap();
                    }
                    None => {
                        // No more work to do
                        println!("Worker {} finished", worker_id);
                        break;
                    }
                }
            }
        });

        handles.push(handle);
    }

    // Drop the original sender so rx will stop when all workers are done
    drop(tx);

    // Collect results as they come in
    let mut results = vec![];
    for result in rx {
        println!("Got result for task {}: {}", result.id, result.result);
        results.push(result);
    }

    // Wait for all workers to finish
    for handle in handles {
        handle.join().unwrap();
    }

    println!("All work completed! Processed {} items", results.len());
}
