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

use crate::data::Todo;

pub fn draw_todo_modal(f: &mut Frame, area: Rect, todo: &Todo) {
    // Futuristic color palette
    let background = Color::Rgb(15, 20, 30); // Deep space blue
    let accent = Color::Rgb(0, 200, 255); // Cyber blue
    let border = Color::Rgb(100, 255, 255); // Light cyan
    let text_primary = Color::Rgb(220, 220, 220); // Off-white
    let text_secondary = Color::Rgb(180, 180, 180); // Light gray

    // Main modal block with futuristic styling
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

    // Create styled text with modern color scheme
    let text = vec![
        Line::from(vec![
            "ID: ".fg(text_secondary),
            todo.id.to_string().bold().fg(accent),
        ]),
        Line::from(""),
        Line::from(vec![
            "PRIORITY: ".fg(text_secondary),
            match todo.priority.to_lowercase().as_str() {
                "high" => todo.priority.as_str().bold().fg(Color::Rgb(255, 50, 100)), // Neon pink-red
                "medium" => todo.priority.as_str().bold().fg(Color::Rgb(255, 200, 0)), // Amber
                _ => todo.priority.as_str().bold().fg(Color::Rgb(50, 255, 100)),      // Neon green
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
                "Done" | "Completed" => todo.status.as_str().bold().fg(Color::Rgb(50, 255, 100)), // Neon green
                "In Progress" => todo.status.as_str().bold().fg(Color::Rgb(255, 200, 0)), // Amber
                "Planned" => todo.status.as_str().bold().fg(accent),
                "Backlog" => todo.status.as_str().bold().fg(Color::Rgb(255, 50, 100)), // Neon pink-red
                _ => todo.status.as_str().bold().fg(accent),
            },
        ]),
        Line::from(""),
        Line::from(vec![
            "CREATED: ".fg(text_secondary),
            todo.date_added.as_str().bold().fg(text_primary),
        ]),
        Line::from(""),
        Line::from("DESCRIPTION:".fg(text_secondary)),
        Line::from(""),
        Line::from(todo.text.as_str().fg(text_primary)),
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
