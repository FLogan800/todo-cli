use clap::{Parser, Subcommand};
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

const TASKS_FILE_NAME: &str = "tasks.json";

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

        #[arg(short, long)]
        description: Option<String>,

        #[arg(short, long)]
        class: Option<String>,
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

    /// Delete all tasks
    Clear,
}

#[derive(Serialize, Deserialize)]
struct Task {
    id: u32,
    title: String,
    description: Option<String>,
    class: Option<String>,
    complete: bool,
}

fn main() {
    let tasks_file_path = get_tasks_file_path().expect("Failed to get task file path");

    let mut task_list = load_tasks(&tasks_file_path);
    
    let cli = Cli::parse();

    match &cli.command {
        Commands::New {
            title,
            description,
            class,
        } => new_task(&mut task_list, title, description, class),
        Commands::List { all, complete } => list_tasks(&task_list, *all, *complete),
        Commands::Complete { id } => mark_task_complete(&mut task_list, *id),
        Commands::Delete { id } => delete_task(&mut task_list, *id),
        Commands::Clear => clear_tasks(&mut task_list),
    }

    save_tasks(&task_list, &tasks_file_path);
}

fn get_tasks_file_path() -> io::Result<PathBuf> {
    let mut tasks_file_path = home_dir().ok_or(io::Error::new(io::ErrorKind::NotFound, "Home directory not found"))?;
    tasks_file_path.push(TASKS_FILE_NAME);
    Ok(tasks_file_path)
}

fn load_tasks(tasks_file_path: &PathBuf) -> Vec<Task> {
    if Path::new(tasks_file_path).exists() {
        let tasks_file = File::open(tasks_file_path).expect("Failed to open file");

        serde_json::from_reader(tasks_file).expect("Failed to extract tasks from file")
    } else {
        Vec::<Task>::new()
    }
}

fn save_tasks(task_list: &Vec<Task>, tasks_file_path: &PathBuf) {
    let task_file: File = File::create(tasks_file_path).expect("Failed to open tasks file");

    serde_json::to_writer_pretty(task_file, &task_list).expect("Failed to write to file");
}

fn new_task(
    task_list: &mut Vec<Task>,
    title: &str,
    description: &Option<String>,
    class: &Option<String>,
) {
    let task_id = if task_list.is_empty() {
        1
    } else {
        task_list[task_list.len() - 1].id + 1
    };

    let task = Task {
        id: task_id,
        title: title.to_string(),
        description: description.clone(),
        class: class.clone(),
        complete: false,
    };

    task_list.push(task);
}

fn list_tasks(task_list: &Vec<Task>, list_all: bool, list_complete: bool) {
    if task_list.is_empty() {
        println!("There are no tasks to display");
        return;
    }

    for task in task_list {
        if list_all || task.complete == list_complete {
            println!("Task ID: {}", task.id);
            println!("Task: {}", task.title);
            if task.class.is_some() {
                println!("Class: {}", task.class.as_deref().unwrap_or_default());
            }
            if task.description.is_some() {
                println!("Description: {}", task.description.as_deref().unwrap());
            }
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

fn clear_tasks(task_list: &mut Vec<Task>) {
    *task_list = Vec::<Task>::new();
}
