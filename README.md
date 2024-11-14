# TODO APP (CMD)

## For using this repo as well as test it follow few steps to setup this application.

### Step 1. Install Rust if is not installed ?
```bash 

    # Windows for Linux..
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

```

### Step 2. Clone this repo.
```bash

    # Open the Directory when you want to clone this application.
    cd <directory name>
    git init
    git clone <repo link>

    # Open this cloned project in your VS Code and any other Code Editor.
    code .

```

### Step 3. Now Run these command to run this project.

```bash

    # you need to run this project with the help of cargo (package manager).
    cargo build
    cargo run

```

### Step 4. ADD, REMOVE, TASK STATUS, TASK LIST Commands run and show outputs.

#### After Run this "cargo run" command then you can see this.

```bash 
    # Task Menu
    Todo List Manager
    Total Tasks: 0 | Completed Tasks: 0 | Pending Tasks: 0
    -------------------MENU---------------------
    1. Add task
    2. Remove task
    3. Toggle task status
    4. List tasks
    5. Exit
    
```
#### Now You can choose any option: 

```bash
    
    # Add Task: 
    Enter your choice (1-5): 1
    Enter task description: Buy Car 
    Task added successfully!

    Todo List:
    ------------------
    1. [!] Buy Car
    ------------------

    Todo List Manager
    Total Tasks: 1 | Completed Tasks: 0 | Pending Tasks: 1
    -------------------MENU---------------------
    1. Add task
    2. Remove task
    3. Toggle task status
    4. List tasks
    5. Exit

```

```bash

    # Remove Task:
    Enter your choice (1-5): 2
    Enter task ID to remove: 4
    Task removed successfully!

    Todo List:
    ------------------
    3. [!] Buy TV
    1. [!] Buy Car
    2. [!] Buy Smartphone
    ------------------

    Todo List Manager
    Total Tasks: 3 | Completed Tasks: 0 | Pending Tasks: 3
    -------------------MENU---------------------
    1. Add task
    2. Remove task
    3. Toggle task status
    4. List tasks
    5. Exit

```

```bash

    # Task Status Toggle:
    Enter your choice (1-5): 3
    Enter task ID to toggle: 2
    Task status toggled successfully!

    Todo List:
    ------------------
    3. [!] Buy TV
    1. [!] Buy Car
    2. [✓] Buy Smartphone
    ------------------

    Todo List Manager
    Total Tasks: 3 | Completed Tasks: 1 | Pending Tasks: 2
    -------------------MENU---------------------
    1. Add task
    2. Remove task
    3. Toggle task status
    4. List tasks
    5. Exit

```

```bash

    # Task List:
    Enter your choice (1-5): 4

    Todo List:
    ------------------
    3. [!] Buy TV
    1. [!] Buy Car
    2. [✓] Buy Smartphone
    ------------------

    Todo List Manager
    Total Tasks: 3 | Completed Tasks: 1 | Pending Tasks: 2
    -------------------MENU---------------------
    1. Add task
    2. Remove task
    3. Toggle task status
    4. List tasks
    5. Exit

```
```bash

    # Exit:
    Enter your choice (1-5): 5
    Goodbye!, Have a good day...

    # Done it This is the Demo..

```

### Try Out Now.
