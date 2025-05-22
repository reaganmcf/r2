use crate::utils::state::StateManager;
use anyhow::Result;
use dialoguer::Select;
use std::io::{self};

pub fn handle_remove() -> Result<()> {
    let todos = StateManager::list_todos().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    if todos.is_empty() {
        println!("No todos found");
        return Ok(());
    }

    // Create a list of formatted todo options
    let todo_options: Vec<String> = todos
        .iter()
        .enumerate()
        .map(|(i, todo)| {
            format!(
                "{}: {}\n  Description: {}\n  Created: {}",
                i + 1,
                todo.title,
                todo.description.as_deref().unwrap_or("None"),
                todo.created_at
            )
        })
        .collect();

    // Show the selection interface
    let selection = Select::new()
        .with_prompt("Select a todo to remove")
        .items(&todo_options)
        .default(0)
        .interact()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    // Get the selected todo's index
    let index = selection + 1;

    // Remove the todo
    StateManager::remove_todo(selection)?;
    println!("Todo {} removed successfully!", index);

    Ok(())
}
