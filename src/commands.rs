use clap::{Parser, Subcommand};

use crate::todos::Todo;
use crate::{add_todo, remove_todo, save_and_exit, show_todos, update_todo};

#[derive(Parser)]
#[command(author,version,about,long_about = None)]
struct Cli {
    #[command(subcommand)]
    sub_commands: Option<TodoCommands>,
}

// List of commands
#[derive(Subcommand)]
enum TodoCommands {
    /// Show todos
    Show,
    /// Add todo
    Add,
    /// Update todo
    Update,
    /// Remove todo
    Remove,
}

/// Execute a command from the command line
pub fn execute_commands(todos: &mut Vec<Todo>) -> Result<(), Box<dyn std::error::Error>> {
    let cm = Cli::parse();
    match &cm.sub_commands {
        Some(TodoCommands::Show) => {
            show_todos(todos);
            save_and_exit(todos);
            Ok(())
        }
        Some(TodoCommands::Add) => {
            add_todo(todos)?;
            save_and_exit(todos);
            Ok(())
        }

        Some(TodoCommands::Update) => {
            update_todo(todos)?;
            save_and_exit(todos);
            Ok(())
        }
        Some(TodoCommands::Remove) => {
            remove_todo(todos)?;
            save_and_exit(todos);
            Ok(())
        }
        None => Ok(()),
    }
}
