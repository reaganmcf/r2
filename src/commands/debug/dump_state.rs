use crate::utils::state::StateManager;
use anyhow::Result;
use serde_json::to_string_pretty;

pub fn handle_dump_state() -> Result<()> {
    let state = StateManager::load()?;
    let json = to_string_pretty(&state)?;
    println!("{}", json);
    Ok(())
}
