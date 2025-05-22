use serde_json::to_string_pretty;
use crate::utils::state::StateManager;
use anyhow::Result;

pub fn handle_dump_state() -> Result<()> {
    let state = StateManager::load()?;
    let json = to_string_pretty(&state)?;
    println!("{}", json);
    Ok(())
}
