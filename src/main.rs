use clap::Parser;
use r2::{commands, utils};

#[derive(Parser, Debug)]
#[command(name = "r2")]
#[command(about = "My personal right-hand man CLI tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser, Debug)]
enum Commands {
    /// Manage your todos
    Todos(TodosCommand),
    /// Debug commands
    Debug(DebugCommand),
}

#[derive(Parser, Debug)]
struct DebugCommand {
    /// Dump the application state to JSON
    #[command(subcommand)]
    command: DebugSubcommand,
}

#[derive(Parser, Debug)]
enum DebugSubcommand {
    /// Dump the state as JSON
    DumpState,
}

#[derive(Parser, Debug)]
struct TodosCommand {
    #[command(subcommand)]
    command: TodosSubcommand,
}

#[derive(Parser, Debug)]
enum TodosSubcommand {
    /// List all todos
    List,
    /// Add a new todo
    Add,
    /// Remove a todo
    Remove,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Todos(todos) => match &todos.command {
            TodosSubcommand::List => {
                if let Err(e) = commands::todos::list::handle_list() {
                    eprintln!("Error listing todos: {}", e);
                }
            }
            TodosSubcommand::Add => {
                if let Err(e) = commands::todos::add::handle_add() {
                    eprintln!("Error adding todo: {}", e);
                }
            }
            TodosSubcommand::Remove => {
                if let Err(e) = commands::todos::remove::handle_remove() {
                    eprintln!("Error removing todo: {}", e);
                }
            }
        },
        Commands::Debug(debug) => match &debug.command {
            DebugSubcommand::DumpState => {
                if let Err(e) = commands::debug::dump_state::handle_dump_state() {
                    eprintln!("Error dumping state: {}", e);
                }
            }
        },
    }
}
