use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;

const TASKS_FILE_PATH: &str = "./tasks.json";

// Command line Parser Configuration
#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(next_line_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new task
    New {
        title: String,
        description: Option<String>,
    },

    /// List tasks, only incomplete by default
    #[group(multiple = false)]
    List {
        /// List all tasks, including completed
        #[arg(short, long)]
        all: bool,

        /// List only completed tasks
        #[arg(short, long)]
        complete: bool,
    },

    /// Mark a task as complete
    Complete { id: u32 },

    /// Delete a task
    Delete { id: u32 },
}

#[derive(Serialize, Deserialize)]
struct Task {
    id: u32,
    title: String,
    description: Option<String>,
    complete: bool,
}

fn main() {
    let cli = Cli::parse();

    let mut task_list: Vec<Task> = if Path::new(TASKS_FILE_PATH).exists() {
        let tasks_file = File::open(TASKS_FILE_PATH).expect("Failed to open file");

        serde_json::from_reader(tasks_file).expect("Failed to extract tasks from file")
    } else {
        Vec::<Task>::new()
    };

    match &cli.command {
        Commands::New { title, description } => {
            new_task(&mut task_list, title.clone(), description.clone())
        }
        Commands::List { all, complete } => list_tasks(&task_list, *all, *complete),
        Commands::Complete { id } => mark_task_complete(&mut task_list, *id),
        Commands::Delete { id } => delete_task(&mut task_list, *id),
    }

    let task_file = File::create(TASKS_FILE_PATH).expect("Failed to open tasks file");

    serde_json::to_writer_pretty(task_file, &task_list).expect("Failed to write to file");
}

fn new_task(task_list: &mut Vec<Task>, title: String, description: Option<String>) {
    let task_id = if task_list.len() == 0 {
        1
    } else {
        task_list[task_list.len() - 1].id + 1
    };

    let task = Task {
        id: task_id,
        title: title.clone(),
        description: description,
        complete: false,
    };

    task_list.push(task);
}

fn list_tasks(task_list: &Vec<Task>, list_all: bool, list_complete: bool) {
    if task_list.len() == 0 {
        println!("There are no tasks to display");
        return;
    }

    for task in task_list {
        if list_all || task.complete == list_complete {
            println!("Task ID: {}", task.id);
            println!("Task: {}", task.title);
            println!(
                "Description: {}",
                task.description.as_deref().unwrap_or_default()
            );
            println!("Complete: {}\n", task.complete);
        }
    }
}

fn mark_task_complete(task_list: &mut Vec<Task>, id: u32) {
    for task in task_list {
        if task.id == id {
            task.complete = true;
            return;
        }
    }

    println!("A task with the given ID does not exist");
}

fn delete_task(task_list: &mut Vec<Task>, id: u32) {
    for index in 0..task_list.len() {
        let task = &task_list[index];

        if task.id == id {
            task_list.remove(index);
            return;
        }
    }

    println!("A task with the given ID does not exist");
}
