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
    #[arg(short, long, value_name = "TEXT", num_args = 1.., value_delimiter = ' ')]
    pub add: Option<Vec<String>>,

    /// Topic for the new todo item
    #[arg(short, long, value_name = "TOPIC", requires = "add")]
    pub topic: Option<String>,

    // Print out all the todos into console
    #[arg(short, long)]
    pub print: bool,
}
