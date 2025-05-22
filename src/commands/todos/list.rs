use crate::utils::state::StateManager;
use std::io::{self};

pub fn handle_list() -> io::Result<()> {
    let todos = StateManager::list_todos().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    if todos.is_empty() {
        println!("No todos found");
        return Ok(());
    }

    println!("Todos:");
    for (i, todo) in todos.iter().enumerate() {
        println!("{}: {}", i + 1, todo.title);
        if let Some(desc) = &todo.description {
            println!("  Description: {}", desc);
        }
        println!("  Created: {}", todo.created_at);
        println!();
    }

    Ok(())
}
