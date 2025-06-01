use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(version)]
struct CLIArgs {
    /// Use a task store which is different to the default
    #[arg(short, long)]
    store: Option<PathBuf>,
    #[command(subcommand)]
    command: Command,
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
    Move,
    /// Create a new task
    Add,
    /// Update a task definition
    Edit,
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

fn main() {
    let args = CLIArgs::parse();
    dbg!(args);
}
