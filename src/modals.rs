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
    // Define a block for the modal with a dark background and magenta border
    let block = Block::default()
        .title("Todo Details")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Rgb(30, 30, 40))) // Dark blue-gray
        .border_style(Style::default().fg(Color::Magenta));

    let area = centered_rect(60, 60, area);
    f.render_widget(block, area);

    let inner_area = area.inner(Margin {
        vertical: 2,
        horizontal: 3,
    });

    // Create styled text for the modal content
    let text = vec![
        Line::from(vec![
            "ID: ".into(),
            todo.id.to_string().bold().fg(Color::Blue),
        ]),
        Line::from(""),
        Line::from(vec![
            "Priority: ".into(),
            if todo.priority == "High" || todo.priority == "HIGH" || todo.priority == "high" {
                todo.priority.as_str().bold().fg(Color::Red)
            } else if todo.priority == "Medium" {
                todo.priority.as_str().bold().fg(Color::Yellow)
            } else {
                todo.priority.as_str().bold().fg(Color::Green)
            },
        ]),
        Line::from(""),
        Line::from(vec![
            "Topic: ".into(),
            todo.topic.as_str().bold().fg(Color::Blue),
        ]),
        Line::from(""),
        Line::from(vec![
            "Status: ".into(),
            match todo.status.as_str() {
                "Done" => todo.status.as_str().bold().fg(Color::Green),
                "Completed" => todo.status.as_str().bold().fg(Color::Green),
                "In Progress" => todo.status.as_str().bold().fg(Color::Yellow),
                "Planned" => todo.status.as_str().bold().fg(Color::Blue),
                "Backlog" => todo.status.as_str().bold().fg(Color::Red),
                _ => todo.status.as_str().bold().fg(Color::Blue),
            },
        ]),
        Line::from(""),
        Line::from(vec![
            "Date Added: ".into(),
            todo.date_added.as_str().bold().fg(Color::Blue),
        ]),
        Line::from(""),
        Line::from("Description:"),
        Line::from(""),
        Line::from(todo.text.as_str().bold().fg(Color::Blue)),
    ];

    // Create a paragraph with the styled text
    let paragraph = Paragraph::new(text)
        .wrap(Wrap { trim: true })
        .block(Block::default());

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
