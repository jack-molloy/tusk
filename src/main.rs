use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(version)]
struct CLIArgs {
    /// Use a task store which is different to the default
    #[arg(short, long)]
    store: Option<PathBuf>,
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Display a list of tasks
    #[clap(visible_alias("ls"))]
    Show {
        /// Whether to show all tasks, or just active ones (default)
        #[arg(short, long)]
        all: bool,
    },
    /// Change the status of a task
    #[clap(visible_alias("mv"))]
    Move { task_id: i32, status: String },
    /// Create a new task
    Add,
    /// Update a task definition
    Edit { task_id: i32 },
    /// Create, view, or change task store
    Store(StoreArgs),
}

#[derive(Debug, Args)]
struct StoreArgs {
    #[command(subcommand)]
    command: Option<StoreCommand>,
}

#[derive(Subcommand, Debug)]
enum StoreCommand {
    /// Create a new task store
    New {
        /// Location of new task store
        #[arg(required(true))]
        path: PathBuf,
    },
    /// Show the current task store
    #[clap(alias = "show")]
    View,
    /// Change the current task store. Future commands will default to this store.
    Switch {
        /// Location of the task store to use
        #[arg(required(true))]
        path: PathBuf,
    },
}

struct Task {
    task_id: i32,
    status: Status,
    title: String,
}

enum Status {
    Backlog,
    Todo,
    Doing,
    Blocked,
    Done,
    Rejected,
}

fn create_task() {
    println!("Creating a new task using an editor of your choice")
}

fn show_active_tasks() {
    println!("Showing active tasks")
}

fn show_all_tasks() {
    println!("Showing all tasks")
}

fn edit_task(task_id: i32) {
    println!("Updating task {} using an editor of your choice", task_id)
}

fn update_task(task_id: i32, status: String) {
    println!("Updating the status of task {} to {}", task_id, status)
}

fn create_store(path: PathBuf) {
    println!("Creating a new task store at {:?}", path)
}

fn get_current_store_path() -> PathBuf {
    return PathBuf::from("./default.tusk");
}

fn print_active_store() {
    println!("The current active store is {:?}", get_current_store_path())
}

fn set_active_store(path: PathBuf) {
    println!("Switching active task store to {:?}", path)
}

fn main() {
    let args = CLIArgs::parse();
    if let Some(command) = args.command {
        match command {
            Command::Show { all } => {
                if all {
                    show_all_tasks();
                } else {
                    show_active_tasks();
                }
            }
            Command::Add => {
                create_task();
            }
            Command::Edit { task_id } => {
                edit_task(task_id);
            }
            Command::Move { task_id, status } => {
                update_task(task_id, status);
            }
            Command::Store(store) => match store.command {
                Some(store_command) => match store_command {
                    StoreCommand::New { path } => {
                        create_store(path);
                    }
                    StoreCommand::View => {
                        print_active_store();
                    }
                    StoreCommand::Switch { path } => {
                        set_active_store(path);
                    }
                },
                None => {
                    print_active_store();
                }
            },
        }
    } else {
        show_all_tasks();
    }
}
