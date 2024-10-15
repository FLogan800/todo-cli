use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};

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

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    title: String,
    description: Option<String>,
    complete: bool,
}

fn main() {
    let cli = Cli::parse();

    let mut task_list: Vec<Task> = vec![];

    match &cli.command {
        Commands::New { title, description } => {
            new_task(&mut task_list, title.clone(), description.clone());
            for task in task_list {
                println!("{:?}", task);
            }
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