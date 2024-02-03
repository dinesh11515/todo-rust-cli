use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author="Dinesh", version="1.0.0", about, long_about = None)]
pub struct Args {
    // /// Name of the person to greet
    // #[arg(short, long)]
    // pub name: String,
    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    pub count: u8,

    #[command(subcommand)]
    pub command: Option<Commands>,
}
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Adds a new task
    Add {
        /// Task name
        #[arg(short, long)]
        task: Vec<String>,
    },
    /// Edits the task name
    Edit {
        /// Old Task name which need to change
        #[arg(short, long)]
        old_task: String,
        /// New task name
        #[arg(short, long)]
        new_task: String,
    },
    /// Removes the task
    Remove {
        /// Task name
        #[arg(short, long)]
        task: String,
    },
    /// Marks the task as done
    Done {
        /// Task name
        #[arg(short, long)]
        task: String,
    },
    List {},
}
