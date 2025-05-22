use dialoguer::{Input, theme::ColorfulTheme};
use crate::utils::state::StateManager;
use anyhow::Result;

pub fn handle_add() -> Result<()> {
    let theme = ColorfulTheme::default();
    
    let title = Input::<String>::with_theme(&theme)
        .with_prompt("Todo title")
        .interact()?;
    
    let description = Input::<String>::with_theme(&theme)
        .with_prompt("Todo description (optional)")
        .allow_empty(true)
        .interact()?;
    
    StateManager::add_todo(title, if description.is_empty() { None } else { Some(description) })?;
    
    println!("Todo added successfully!");
    Ok(())
}
