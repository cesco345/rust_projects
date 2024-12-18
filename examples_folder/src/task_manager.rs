use std::collections::HashMap;

#[derive(Debug)]
enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Debug)]
enum Status {
    Todo,
    InProgress(String), // Stores assignee
    Done(String),       // Stores completion date
    Blocked(String),    // Stores reason
}

#[derive(Debug)]
struct Task {
    description: String,
    priority: Priority,
    status: Status,
}

fn main() {
    let mut tasks = HashMap::new();

    // Create some sample tasks
    tasks.insert(
        1,
        Task {
            description: String::from("Implement login system"),
            priority: Priority::High,
            status: Status::InProgress(String::from("Alice")),
        },
    );

    tasks.insert(
        2,
        Task {
            description: String::from("Update documentation"),
            priority: Priority::Low,
            status: Status::Todo,
        },
    );

    tasks.insert(
        3,
        Task {
            description: String::from("Fix security bug"),
            priority: Priority::High,
            status: Status::Blocked(String::from("Waiting for security audit")),
        },
    );

    // Process tasks using if let
    for (id, task) in &tasks {
        print!("Task {}: {} - ", id, task.description);

        // Check high priority tasks
        if let Priority::High = task.priority {
            print!("[URGENT] ");
        }

        // Print status with details
        match &task.status {
            Status::Todo => println!("Not started"),
            Status::InProgress(assignee) => println!("Being worked on by {}", assignee),
            Status::Done(date) => println!("Completed on {}", date),
            Status::Blocked(reason) => println!("Blocked: {}", reason),
        }

        // Special handling for blocked high-priority tasks
        if let (Priority::High, Status::Blocked(reason)) = (&task.priority, &task.status) {
            println!("⚠️ ATTENTION: High priority task is blocked: {}", reason);
        }
    }
}
