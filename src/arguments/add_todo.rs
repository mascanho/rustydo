// In arguments/add_todo.rs
use crate::{data::Todo, database::DBtodo};
use chrono::Local;
use std::error::Error;

pub fn add_todo(
    text: String,
    topic: Option<String>,
    priority: Option<String>,
) -> Result<(), Box<dyn Error>> {
    let date_added = Local::now().format("%Y-%m-%d").to_string();
    let topic = topic.unwrap_or_else(|| "General".to_string());

    let priority = priority.unwrap_or_else(|| "normal".to_string());

    let db = DBtodo::new()?;

    let new_todo = Todo {
        id: 0, // Will be auto-incremented by SQLite
        priority,
        topic,
        text,
        date_added,
        status: "Backlog".to_string(),
    };

    db.add_todo(&new_todo)?;
    Ok(())
}
