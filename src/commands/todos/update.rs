use crate::utils::state::StateManager;
use crate::utils::state::Todo;
use anyhow::Result;
use dialoguer::{Input, Select};

pub fn handle_update() -> Result<()> {
    let todos =
        StateManager::list_todos().map_err(|e| anyhow::anyhow!("Failed to list todos: {}", e))?;

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
        .with_prompt("Select a todo to update")
        .items(&todo_options)
        .default(0)
        .interact()
        .map_err(|e| anyhow::anyhow!("Failed to show selection: {}", e))?;

    // Get the selected todo
    let selected_todo = &todos[selection];

    // Get new title
    let new_title = Input::<String>::new()
        .with_prompt("New title (leave blank to keep current)")
        .default(selected_todo.title.clone())
        .interact()
        .map_err(|e| anyhow::anyhow!("Failed to get new title: {}", e))?;

    // Get new description
    let new_description = Input::<String>::new()
        .with_prompt("New description (leave blank to keep current)")
        .default(
            selected_todo
                .description
                .clone()
                .unwrap_or_else(|| String::new()),
        )
        .interact()
        .map_err(|e| anyhow::anyhow!("Failed to get new description: {}", e))?;

    // Update the todo
    StateManager::update_todo(
        selection,
        new_title,
        if new_description.is_empty() {
            None
        } else {
            Some(new_description)
        },
    )
    .map_err(|e| anyhow::anyhow!("Failed to update todo: {}", e))?;

    println!("Todo updated successfully!");
    Ok(())
}
