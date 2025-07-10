use crate::App;
use crate::data::Todo;
use crate::modals::{centered_rect, draw_todo_modal};
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

pub fn draw_ui(f: &mut Frame, app: &mut App) {
    let area = f.area();

    // Handle modal and delete confirmation states first (highest priority)
    if app.show_delete_confirmation {
        draw_delete_confirmation(f, area);
        return; // Early return to prevent drawing other UI elements
    }

    if app.show_modal {
        draw_todo_modal(f, area, app.selected_todo.as_ref().unwrap());
        return; // Early return to prevent drawing other UI elements
    }

    // Main table view (only reached if neither modal nor confirmation is showing)
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Min(1),    // Main table area
            Constraint::Length(3), // Stats area
            Constraint::Length(1), // Shortcuts area
        ])
        .split(area);

    let header = Row::new(vec![
        "ID",
        "Priority",
        "Topic",
        "Text",
        "Date Added",
        "Status",
    ])
    .style(
        Style::default()
            .fg(Color::Blue)
            .add_modifier(Modifier::BOLD),
    );

    let rows = app.todos.iter().map(|todo| {
        Row::new(vec![
            todo.id.to_string(),
            todo.priority.clone(),
            todo.topic.clone(),
            todo.text.clone(),
            todo.date_added.clone(),
            todo.status.clone(),
        ])
    });

    let table = Table::new(
        rows.collect::<Vec<_>>(),
        vec![
            Constraint::Length(5),
            Constraint::Length(20),
            Constraint::Length(15),
            Constraint::Percentage(35),
            Constraint::Length(12),
            Constraint::Length(10),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .title("📝 TODO List")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Magenta)),
    )
    .highlight_style(Style::default().bg(Color::DarkGray).fg(Color::White))
    .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED))
    .column_spacing(2);

    f.render_stateful_widget(table, layout[0], &mut app.state);

    let (completed, in_progress, planned, backlog) = calculate_stats(&app.todos);
    let stats_text = format!(
        "Total: {}, Completed: {}, In Progress: {}, Planned: {}, Backlog: {}",
        app.todos.len(),
        completed,
        in_progress,
        planned,
        backlog
    );

    let status_line = Paragraph::new(stats_text)
        .style(Style::default().fg(Color::Magenta))
        .block(
            Block::default()
                .borders(Borders::empty())
                .style(Style::default().fg(Color::Magenta)),
        );

    f.render_widget(status_line, layout[1]);

    // Render shortcuts
    let shortcuts =
        Paragraph::new(get_shortcuts_text()).style(Style::default().fg(Color::DarkGray));

    f.render_widget(shortcuts, layout[2]);
}

pub fn calculate_stats(todos: &[Todo]) -> (usize, usize, usize, usize) {
    let completed = todos
        .iter()
        .filter(|todo| todo.status == "Completed")
        .count();
    let in_progress = todos
        .iter()
        .filter(|todo| todo.status == "In Progress")
        .count();
    let planned = todos.iter().filter(|todo| todo.status == "Planned").count();
    let backlog = todos.iter().filter(|todo| todo.status == "Backlog").count();
    (completed, in_progress, planned, backlog)
}

// KEYWBOARD SHORTCUTS
fn get_shortcuts_text() -> Line<'static> {
    Line::from(vec![
        "↑/↓: Navigate".into(),
        " ".into(),
        "Enter: View".into(),
        " ".into(),
        "d: Delete".into(),
        " ".into(),
        "q: Quit".into(),
    ])
}

// Delete MOdal
fn draw_delete_confirmation(f: &mut Frame, area: Rect) {
    let block = Block::default()
        .title(" Confirm Delete ")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::DarkGray))
        .border_style(Style::default().fg(Color::Red));

    let area = centered_rect(40, 20, area);
    f.render_widget(block, area);

    let text = vec![
        Line::from("Are you sure you want to delete this item?"),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "Y",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::from(": Yes, delete"),
        ]),
        Line::from(vec![
            Span::styled(
                "N",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::from(": Cancel"),
        ]),
    ];

    let paragraph = Paragraph::new(text)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}
