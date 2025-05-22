use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Cell, Row, Table},
    Terminal,
};
use std::io::{self};
use crate::utils::state::StateManager;

pub fn handle_list() -> io::Result<()> {
    let todos = StateManager::list_todos()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    
    if todos.is_empty() {
        let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
        terminal.clear()?;
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(1),  // Just enough space for header
                ])
                .split(size);

            let table = Table::new(vec![], &[
                Constraint::Length(3),  // Index column
                Constraint::Length(20), // Title column
                Constraint::Length(30), // Description column
                Constraint::Length(20), // Created column
            ])
            .header(Row::new(vec![
                Cell::from("#"),
                Cell::from("Title"),
                Cell::from("Description"),
                Cell::from("Created"),
            ]))
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Todos")
                .title_style(Style::default().fg(Color::Yellow))
                .style(Style::default().fg(Color::White)));

            f.render_widget(table, chunks[0]);
        })?;
        println!("No todos found");
        return Ok(());
    }

    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    terminal.clear()?;
    terminal.draw(|f| {
        let size = f.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1 + todos.len() as u16),  // Header + rows
            ])
            .split(size);

        let rows: Vec<Row> = todos
            .iter()
            .enumerate()
            .map(|(i, todo)| {
                Row::new(vec![
                    Cell::from(format!("{}", i + 1)).style(Style::default().fg(Color::Yellow)),
                    Cell::from(todo.title.clone()),
                    Cell::from(todo.description.clone().unwrap_or_default()),
                    Cell::from(todo.created_at.clone()),
                ])
            })
            .collect();

        let table = Table::new(rows, &[
                Constraint::Length(3),  // Index column
                Constraint::Length(20), // Title column
                Constraint::Length(30), // Description column
                Constraint::Length(20), // Created column
            ])
            .header(Row::new(vec![
                Cell::from("#"),
                Cell::from("Title"),
                Cell::from("Description"),
                Cell::from("Created"),
            ]))
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Todos")
                .title_style(Style::default().fg(Color::Yellow))
                .style(Style::default().fg(Color::White)));

        f.render_widget(table, chunks[0]);
    })?;

    Ok(())
}
