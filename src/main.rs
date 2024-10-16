use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::path::Path;

const TASKS_FILE_PATH: &str = "tasks.json";

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

    /// List all tasks
    List,

    /// Mark a task as complete
    Complete,

    /// Delete a task
    Delete,
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
    }
    else {
        Vec::<Task>::new()
    };

    match &cli.command {
        Commands::New { title, description } => {
            new_task(&mut task_list, title.clone(), description.clone());

            let task_file = File::create(TASKS_FILE_PATH).expect("Failed to open tasks file");

            serde_json::to_writer_pretty(task_file, &task_list).expect("Failed to write to file");  
        },
        Commands::List => {
            list_tasks(&task_list);
        },
        _ => println!("Not implemented yet"),
    }
}

fn new_task(task_list: &mut Vec<Task>, title: String, description: Option<String>) {
    let task_id = if task_list.len() == 0 { 1 } else { task_list[task_list.len() - 1].id + 1 };

    let task = Task {
        id: task_id,
        title: title.clone(),
        description: description,
        complete: false,
    };

    task_list.push(task);
}

fn list_tasks(task_list: &Vec<Task>) {
    for task in task_list {
        println!("Task ID: {}", task.id);
        println!("Task: {}", task.title);
        println!("Description: {}", task.description.as_deref().unwrap_or_default());
        println!("Complete: {}\n", task.complete);
    }
}