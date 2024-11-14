// use::std::collections::HashMap;
// use::std::io::{self, Write};

// // Task Structure Designed
// struct Task {
//     id: i32,
//     description: String,
//     task_status: bool
// }

// // TaskList Structure Design
// struct TodoList {
//     tasks: HashMap<i32, Task>, 
//     task_id: i32
// }

// // Create TodoList like a Class..
// impl TodoList {

//     // Add New Function for add the Todo inside it.
//     fn new() -> TodoList { 
//         TodoList {
//             tasks: HashMap::new(),
//             task_id: 1
//         }        
//     }
//     // Add Task Function
//     fn add_tasks (&mut self , description: String){
//         let task: Task = Task{
//             task_id: self.task_id,
//             description,
//             task_status: false,
//         };
//         self.tasks.insert(self.task_id, description);
//         self.task_id += 1;
//         println!("Task Added Successfully!");
//     }
//     // Task Remove Function
//     fn remove_tasks (&mut self, id:i32) -> bool {
//         if self.tasks.remove(&id).is_some() {
//             println!("Task Removed Successfully!");
//             true
//         }else{
//             println!("Task not found?");
//             false
//         }
//     }
//     // Task Toggle Function (Complete/Pending)
//     fn tasks_toggle (&mut self, id:i32) -> bool {
//        if let Some(task) = self.tasks.get_mut(&id) {
//             task.completed = !task.completed;
//             println!("Task Status Toggled Successfully!");
//             true
//        }else{
//             println!("Task not found?");
//             false
//        }
//     }
//     // Task Lists
//     fn list_tasks (&self){
//         if self.tasks.is_empty() {
//             println!("Task not found!");
//             return
//         }

//         println!("\nTodo List:");
//         println!("------------------");
//         for task in self.tasks.values() {
//             let status = if task.completed { "✓" } else { "!" };
//             println!("{}. [{}] {}", task.id, status, task.description);
//         }
//         println!("------------------");
//     }

//     // All Task Counter Function
//     fn task_count (&self) -> usize {
//         return self.tasks.len();
//     }

//     // Completed Task Counter Function
//     fn completed_tasks (&self) -> i32 {
//         let mut complete = 0;
//         for task in self.tasks.values() {
//             if task.completed == true {
//                 complete += 1;
//             }
//         }
//         return complete;
//     }

//     // Pending Task Counter Function
//     fn pending_tasks (&self) -> i32 {
//         let mut pending = 0;
//         for task in self.tasks.values() {
//             if task.completed == false {
//                 pending += 1;
//             }
//         }
//         return pending;
//     }
// }

// This Program is not completed yet..