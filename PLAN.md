# Todo List CLI - Project Plan

## Project Overview

This project is a CLI application written in Rust for managing a simple todo list.
Users will be able to add and delete tasks, mark tasks complete, as well as save their task list to a file for persistence across sessions

## Project Goals

1. Provide a simple and intuitive CLI for managing tasks
2. Have tasks persist across sessions
3. Use Rust libraries and best practices

## Features

### Core Features

- Add tasks
  - Add a new task to the list with a title, description, and unique ID
- List tasks
  - Display a list of all the tasks to the command line
- Mark as complete
  - Mark a task as complete
- Delete a task
  - Remove a task from the list entirely
- Save and load tasks
  - Save tasks to a file
  - Load tasks from the file when the program starts

### Optional Features

- Due date
  - Set a due date for tasks
- Filter tasks
  - Display tasks filtered by completion, due date, etc...
- Clear tasks
  - Remove every task in the list for a fresh start

## Data Structures

### Task Struct

Each task will be represented by the following struct:

```Rust
struct Task {
  id: u32,
  title: String,
  class: Option<String>,
  description: Option<String>,
  complete: bool,
}
```

- id: Unique identifire for each task
- title: The title of the task
- description: The task's text description
- complete: The completion status of the task

### Task List

A task list vector will be used to store tasks.
This will be serialized to JSON for file storage.

```Rust
task_list: Vec<Task>;
```

## Command Structure

Here is the planned command structure with brief descriptions.

### Core Commands

| Command | Description |
| ------- | ----------- |
| todo new \<title\> [description] [class] | Create a new task with the given description |
| todo list | List all tasks |
| todo complete <task_id> | Mark a task as complete |
| todo delete <task_id> | Remove a task from the list |
| todo --help | Lists all commands |

### Optional Commands

| Command | Description |
| ------- | ----------- |
| todo list --complete | List completed tasks |
| todo list --all | List all tasks |
| todo clear | Deletes all tasks |

## Stack

- Argument Parsing: `clap`
  - For managing command line arguments and options
- Data Storage: `serde` & `serde_json`
  - Serialize the task list and store in a JSON file
- File Handling: `std::fs`
  - To handle read/write operations for saving/loading tasks

## Functionality Outline

```Rust
new_task(description: String) -> Task
```

- Creates a new `Task` with a unique ID and adds it to the task list

```Rust
list_tasks(filter: Option<&str>)
```

- Display tasks with an optional filter for completed/incomplete tasks

```Rust
mark_task_complete(id: u32) -> Result<(), String>
```

- Finds a task by ID sets its `complete` field to `true`

```Rust
delete_task(id: u32) -> Result<(), String>
```

- Finds a task by ID and removes it from `task_list`

## Main Function Flow

1. Parse command-line arguments using `clap`
2. Match the command and call the appropriate function
3. Load the task list from the file at the start of the program
4. After any updates, save the task_list to the file

## Error Handling

- Use `Result` and `Error` types to manage erros, especially for I/O and missing task ID
- Display user friendly error messages for invalid actions (e.g. trying to delete a task with an ID that doesn't exist)

## Future Enhancements

- Due dates
  - Allow users add and filter tasks by due date
- Task Priority
  - Allow priorities (high, medium, low) to be set to tasks
- Improved CLI Output
  - Enhace formatting for a better CLI user experience
