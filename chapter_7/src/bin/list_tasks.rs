use chapter_7::TaskList;

fn main() {
    let task_list = TaskList::new();
    println!("Tasks:");
    for task in task_list.list_tasks() {
        let status = if task.completed { "✓" } else { " " };
        println!("[{}] {}: {}", status, task.id, task.description);
    }
}
