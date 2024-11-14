use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug)]
struct Task {
    id: u32,
    description: String,
    completed: bool,
}

struct TodoList {
    tasks: HashMap<u32, Task>,
    next_id: u32,
}
impl TodoList {
    fn new() -> TodoList {
        TodoList {
            tasks: HashMap::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, description: String) {
        let task = Task {
            id: self.next_id,
            description,
            completed: false,
        };
        self.tasks.insert(self.next_id, task);
        self.next_id += 1;
        println!("Task added successfully!");
    }

    fn remove_task(&mut self, id: u32) -> bool {
        if self.tasks.remove(&id).is_some() {
            println!("Task removed successfully!");
            true
        } else {
            println!("Task not found!");
            false
        }
    }

    fn toggle_task(&mut self, id: u32) -> bool {
        if let Some(task) = self.tasks.get_mut(&id) {
            task.completed = !task.completed;
            println!("Task status toggled successfully!");
            true
        } else {
            println!("Task not found!");
            false
        }
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks found!");
            return;
        }

        println!("\nTodo List:");
        println!("------------------");
        for task in self.tasks.values() {
            let status = if task.completed { "✓" } else { "!" };
            println!("{}. [{}] {}", task.id, status, task.description);
        }
        println!("------------------");
    }

    fn task_count (&self) -> usize {
        return self.tasks.len();
    }

    fn completed_tasks (&self) -> i32 {
        let mut complete = 0;
        for task in self.tasks.values() {
            if task.completed == true {
                complete += 1;
            }
        }
        return complete;
    }

    fn pending_tasks (&self) -> i32 {
        let mut pending = 0;
        for task in self.tasks.values() {
            if task.completed == false {
                pending += 1;
            }
        }
        return pending;
    }

}

fn main() {
    let mut todo_list = TodoList::new();
    
    loop {
        println!("\nTodo List Manager");
        println!("Total Tasks: {} | Completed Tasks: {} | Pending Tasks: {}", todo_list.task_count(), todo_list.completed_tasks(), todo_list.pending_tasks());
        println!("-------------------MENU---------------------");
        println!("1. Add task");
        println!("2. Remove task");
        println!("3. Toggle task status");
        println!("4. List tasks");
        println!("5. Exit");
        print!("\nEnter your choice (1-5): ");


        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                print!("Enter task description: ");
                io::stdout().flush().unwrap();
                let mut description = String::new();
                io::stdin().read_line(&mut description).unwrap();
                todo_list.add_task(description.trim().to_string());
                todo_list.list_tasks();
            }
            "2" => {
                print!("Enter task ID to remove: ");
                io::stdout().flush().unwrap();
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse::<u32>() {
                    todo_list.remove_task(id);
                    todo_list.list_tasks();
                } else {
                    println!("Invalid task ID!");
                }
            }
            "3" => {
                print!("Enter task ID to toggle: ");
                io::stdout().flush().unwrap();
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse::<u32>() {
                    todo_list.toggle_task(id);
                    todo_list.list_tasks();
                } else {
                    println!("Invalid task ID!");
                }
            }
            "4" => {
                todo_list.list_tasks();
            }
            "5" => {
                println!("Goodbye!, Have a good day...");
                break;
            }
            _ => println!("Invalid choice! Please try again."),
        }
    }
}