use chapter_7::TaskList;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: add_task <task description>");
        return;
    }

    let mut task_list = TaskList::new();
    let description = args[1..].join(" ");
    let id = task_list.add_task(description.clone());
    println!("Added task {}: {}", id, description);
}
