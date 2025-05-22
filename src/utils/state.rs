use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;
use anyhow::{Context, Result};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AppState {
    pub todos: Vec<Todo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub title: String,
    pub description: Option<String>,
    pub created_at: String,
}

pub struct StateManager;

impl StateManager {
    fn get_state_path() -> Result<PathBuf> {
        let mut path = dirs::config_dir()
            .context("Failed to get config directory")?
            .join("r2");
        
        fs::create_dir_all(&path)
            .context("Failed to create config directory")?;
            
        path.push("state.json");
        Ok(path)
    }

    pub fn load() -> Result<AppState> {
        let path = Self::get_state_path()?;
        
        if !path.exists() {
            return Ok(AppState::default());
        }

        let data = fs::read_to_string(&path)
            .context("Failed to read state file")?;

        serde_json::from_str(&data)
            .context("Failed to deserialize state")
    }

    pub fn save(state: &AppState) -> Result<()> {
        let path = Self::get_state_path()?;
        
        let data = serde_json::to_string_pretty(state)
            .context("Failed to serialize state")?;
        
        fs::write(&path, data)
            .context("Failed to write state file")
    }

    pub fn add_todo(title: String, description: Option<String>) -> Result<()> {
        let mut state = Self::load()?;
        let todo = Todo {
            title,
            description,
            created_at: chrono::Local::now().to_rfc3339(),
        };
        state.todos.push(todo);
        Self::save(&state)
    }

    pub fn remove_todo(index: usize) -> Result<()> {
        let mut state = Self::load()?;
        if index >= state.todos.len() {
            return Err(anyhow::anyhow!("Todo index out of bounds"));
        }
        state.todos.remove(index);
        Self::save(&state)
    }

    pub fn list_todos() -> Result<Vec<Todo>> {
        let state = Self::load()?;
        Ok(state.todos)
    }
}
