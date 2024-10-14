use clap::{arg, command, Command};

struct _Task {
    id: u32,
    description: String,
    complete: bool,
}

fn main() {
    let _matches = command!()
    .propagate_version(true)
    .next_line_help(true)
    .subcommand_required(true)
    .subcommand(
        Command::new("add")
            .about("Adds a new task with the given description")
            .arg(
                arg!(<DESCRIPTION>)
                    .required(true)
            )
    )
    .subcommand(
        Command::new("list")
            .about("List all tasks")
    )
    .subcommand(
        Command::new("complete")
            .about("Mark a task as complete")
            .arg(
                arg!(<TASK_ID>)
            )
    )
    .subcommand(
        Command::new("delete")
            .about("Remove a task from the list")
            .arg(
                arg!(<TASK_ID>)
            )
    )
    .get_matches();
}
