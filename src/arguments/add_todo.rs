// In arguments/add_todo.rs
use crate::database::DBtodo;
use chrono::Local;
use std::error::Error;

use super::models::Todo;

pub fn add_todo(
    text: String,
    topic: Option<String>,
    priority: Option<String>,
    ownder: Option<String>,
    due: Option<String>,
    desc: Option<String>,
) -> Result<(), Box<dyn Error>> {
    let date_added = Local::now().format("%d-%m-%y").to_string();
    let topic = topic.unwrap_or_else(|| "General".to_string());

    // handle priority
    let priority = priority.unwrap_or_else(|| "normal".to_string());
    if priority != "normal" && priority != "high" && priority != "low" && priority != "medium" {
        return Err("Priority must be 'medium', 'high', or 'low'.".into());
    }
    let priority = priority.to_lowercase();
    // Uppercase only the first letter
    let priority = priority
        .chars()
        .next()
        .unwrap()
        .to_ascii_uppercase()
        .to_string()
        + &priority[1..];

    // Handle the owner string
    let owner = ownder.unwrap_or_else(|| "You".to_string());

    // Ensure the first letter is cased if the user passed argument
    let owner = owner
        .chars()
        .next()
        .unwrap()
        .to_ascii_uppercase()
        .to_string()
        + &owner[1..];

    // ensure the topic is always capital cased on the first letter
    let topic = topic
        .chars()
        .next()
        .unwrap()
        .to_ascii_uppercase()
        .to_string()
        + &topic[1..];

    // Ensure the text first chartacter is always capital cased
    let text = text
        .chars()
        .next()
        .unwrap()
        .to_ascii_uppercase()
        .to_string()
        + &text[1..];

    // Handle the date
    let due_date = due.unwrap_or_else(|| "-".to_string());

    // Ensure the first letter is cased if the user passed argument
    let desc = desc.unwrap_or_else(|| "No description provided".to_string());
    let desc = desc
        .chars()
        .next()
        .unwrap()
        .to_ascii_uppercase()
        .to_string()
        + &desc[1..];

    let db = DBtodo::new()?;

    let new_todo = Todo {
        id: 0, // Will be auto-incremented by SQLite
        priority,
        topic,
        text,
        desc,
        date_added,
        due: due_date,
        status: "Pending".to_string(),
        owner,
    };

    db.add_todo(&new_todo)?;
    Ok(())
}
