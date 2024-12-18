// Declare the geometry module
pub mod geometry;

// Task manager structs and implementations
pub struct Task {
    pub id: u32,
    pub description: String,
    pub completed: bool,
}

pub struct TaskList {
    tasks: Vec<Task>,
}

impl TaskList {
    pub fn new() -> TaskList {
        TaskList { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, description: String) -> u32 {
        let id = self.tasks.len() as u32 + 1;
        self.tasks.push(Task {
            id,
            description,
            completed: false,
        });
        id
    }

    pub fn list_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    pub fn complete_task(&mut self, id: u32) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
            true
        } else {
            false
        }
    }
}
