use clap::{Parser, Subcommand};

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

struct _Task {
    id: u32,
    title: String,
    description: Option<String>,
    complete: bool,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { title, description } => {
            println!("title: {:?}", title);
            println!("description: {:?}", description);
        },
        _ => println!("Not yet"),
    }
}
