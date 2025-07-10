use ratatui::layout::Alignment;
use ratatui::prelude::Stylize;
use ratatui::text::Span;
use ratatui::widgets::TableState;
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph, Row, Table, Wrap},
};

use ratatui::widgets::Clear;

use crate::arguments::models::Todo;

pub fn draw_todo_modal(f: &mut Frame, area: Rect, todo: &Todo) {
    // Elegant purple color palette
    let background = Color::Rgb(25, 15, 30); // Deep purple
    let accent = Color::Rgb(150, 80, 220); // Vibrant purple
    let border = Color::Rgb(180, 140, 220); // Soft lavender
    let text_primary = Color::Rgb(230, 220, 240); // Light lavender
    let text_secondary = Color::Rgb(200, 180, 220); // Muted lavender

    // Main modal block with elegant styling
    let block = Block::default()
        .title(" TODO DETAILS ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border).add_modifier(Modifier::BOLD))
        .style(Style::default().bg(background).fg(text_primary));

    let area = centered_rect(60, 60, area);
    f.render_widget(block, area);

    let inner_area = area.inner(Margin {
        vertical: 2,
        horizontal: 3,
    });

    // Create styled text with purple color scheme
    let text = vec![
        Line::from(vec![
            "ID: ".fg(text_secondary),
            todo.id.to_string().bold().fg(accent),
        ]),
        Line::from(""),
        Line::from(vec![
            "PRIORITY: ".fg(text_secondary),
            match todo.priority.to_lowercase().as_str() {
                "high" => todo.priority.as_str().bold().fg(Color::Rgb(220, 80, 150)), // Pinkish purple
                "medium" => todo.priority.as_str().bold().fg(Color::Rgb(180, 120, 220)), // Medium purple
                _ => todo.priority.as_str().bold().fg(Color::Rgb(120, 80, 200)), // Deep purple
            },
        ]),
        Line::from(""),
        Line::from(vec![
            "TOPIC: ".fg(text_secondary),
            todo.topic.as_str().bold().fg(accent),
        ]),
        Line::from(""),
        Line::from(vec![
            "STATUS: ".fg(text_secondary),
            match todo.status.as_str() {
                "Done" | "Completed" => todo.status.as_str().bold().fg(Color::Rgb(120, 220, 150)), // Soft green
                "In Progress" => todo.status.as_str().bold().fg(Color::Rgb(220, 180, 100)), // Amber
                "Planned" => todo.status.as_str().bold().fg(accent),
                "Pending" => todo.status.as_str().bold().fg(Color::Rgb(220, 100, 120)), // Soft red
                _ => todo.status.as_str().bold().fg(accent),
            },
        ]),
        Line::from(""),
        Line::from(vec![
            "CREATED: ".fg(text_secondary),
            todo.date_added.as_str().bold().fg(text_primary),
        ]),
        Line::from(""),
        Line::from(vec![
            "DUE DATE".fg(text_secondary),
            todo.due.as_str().bold().fg(text_primary),
        ]),
        Line::from(""),
        Line::from(vec![
            "TODO: ".fg(text_secondary),
            todo.text.as_str().bold().fg(text_primary),
        ]),
        Line::from(""),
        Line::from("DESCRIPTION".fg(text_secondary)),
        Line::from(""),
        Line::from(todo.desc.as_str().fg(text_primary)),
    ];

    // Paragraph with subtle styling
    let paragraph = Paragraph::new(text)
        .wrap(Wrap { trim: true })
        .block(Block::default().style(Style::default().bg(background)));

    f.render_widget(paragraph, inner_area);
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

// DELETE CONFIRMATION MODAL
pub fn draw_delete_confirmation(f: &mut Frame, area: Rect) {
    // Purple-themed delete confirmation
    let background = Color::Rgb(30, 15, 35); // Slightly darker purple
    let border = Color::Rgb(200, 100, 220); // Bright purple border for warning
    let text_primary = Color::Rgb(230, 220, 240); // Light lavender
    let text_secondary = Color::Rgb(200, 180, 220); // Muted lavender

    let block = Block::default()
        .title(" Confirm Delete ")
        .borders(Borders::ALL)
        .style(Style::default().bg(background))
        .border_style(Style::default().fg(border).add_modifier(Modifier::BOLD));

    let area = centered_rect(40, 20, area);
    f.render_widget(block, area);

    let text = vec![
        Line::from("Are you sure you want to delete this item?".fg(text_primary)),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "Y",
                Style::default()
                    .fg(Color::Rgb(120, 220, 150)) // Soft green
                    .add_modifier(Modifier::BOLD),
            ),
            Span::from(": Yes, delete".fg(text_secondary)),
        ]),
        Line::from(vec![
            Span::styled(
                "N",
                Style::default()
                    .fg(Color::Rgb(220, 100, 120)) // Soft red
                    .add_modifier(Modifier::BOLD),
            ),
            Span::from(": Cancel".fg(text_secondary)),
        ]),
    ];

    let paragraph = Paragraph::new(text)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .block(Block::default().style(Style::default().bg(background)));

    f.render_widget(paragraph, area);
}

pub fn draw_status_change_modal(f: &mut Frame, area: Rect) {
    let background = Color::Rgb(25, 15, 30); // Deep purple
    let border = Color::Rgb(180, 140, 220); // Soft lavender
    let text_primary = Color::Rgb(230, 220, 240); // Light lavender
    let text_secondary = Color::Rgb(200, 180, 220); // Muted lavender

    let area = centered_rect(40, 20, area);
    f.render_widget(Clear, area); // Clear the modal area before rendering

    let block = Block::default()
        .title(" Change Status ")
        .borders(Borders::ALL)
        .style(Style::default().bg(background))
        .border_style(Style::default().fg(border).add_modifier(Modifier::BOLD));

    f.render_widget(block, area);

    let text = vec![
        Line::from(Span::styled(
            "Change TODO status",
            Style::default().fg(text_primary),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "p",
                Style::default()
                    .fg(Color::Rgb(120, 220, 150))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(": In Progress", Style::default().fg(text_secondary)),
        ]),
        Line::from(vec![
            Span::styled(
                "g",
                Style::default()
                    .fg(Color::Rgb(220, 100, 120))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(": Done", Style::default().fg(text_secondary)),
        ]),
        Line::from(vec![
            Span::styled(
                "p",
                Style::default()
                    .fg(Color::Rgb(220, 100, 120))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(": Pending", Style::default().fg(text_secondary)),
        ]),
    ];

    let paragraph = Paragraph::new(text)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .block(Block::default().style(Style::default().bg(background)));

    f.render_widget(paragraph, area);
}
