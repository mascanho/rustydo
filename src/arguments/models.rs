use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "Todo App")]
#[command(version = "1.0")]
#[command(about = "A simple todo application", long_about = None)]
pub struct Cli {
    /// List all todos in a terminal UI
    #[arg(short, long)]
    pub list: bool,

    /// Add a new todo item
    #[arg(short = 'a', long, value_name = "TEXT", num_args = 1.., value_delimiter = ' ')]
    pub add: Option<Vec<String>>,

    /// Topic for the new todo item (requires --add)
    #[arg(short = 't', long, value_name = "TOPIC", requires = "add")]
    pub topic: Option<String>,

    /// Priority for the todo (requires --add)
    #[arg(short = 'P', long, value_name = "PRIORITY", requires = "add")]
    pub priority: Option<String>,

    /// Print all todos to the console
    #[arg(short = 'p', long)]
    pub print: bool,

    /// Delete a todo by ID
    #[arg(short = 'r', long = "remove", value_name = "ID")]
    pub remove: Option<i32>,

    /// ID of the todo to update
    #[arg(short = 'u', long, value_name = "ID")]
    pub update_id: Option<i32>,

    /// New status for the todo (requires --update-id)
    #[arg(long, value_name = "STATUS", requires = "update_id")]
    pub status: Option<String>,

    /// Mark a todo as done by ID
    #[arg(short = 'd', long = "done", value_name = "ID")]
    pub done: Option<i32>,

    /// Clear all todos
    #[arg(short = 'c', long)]
    pub clear: bool,
}
