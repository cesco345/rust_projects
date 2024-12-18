use chapter_7::TaskList;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: complete_task <task_id>");
        return;
    }

    let task_id = match args[1].parse::<u32>() {
        Ok(id) => id,
        Err(_) => {
            println!("Invalid task ID");
            return;
        }
    };

    let mut task_list = TaskList::new();
    if task_list.complete_task(task_id) {
        println!("Marked task {} as complete", task_id);
    } else {
        println!("Task {} not found", task_id);
    }
}
