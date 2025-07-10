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

    // Futuristic color palette (consistent with modal)
    let background = Color::Rgb(15, 20, 30); // Deep space blue
    let accent = Color::Rgb(0, 200, 255); // Cyber blue
    let border = Color::Rgb(100, 255, 255); // Light cyan
    let text_primary = Color::Rgb(220, 220, 220); // Off-white
    let text_secondary = Color::Rgb(180, 180, 180); // Light gray
    let highlight = Color::Rgb(40, 50, 60); // Slightly lighter than background

    // Handle modal and delete confirmation states first
    if app.show_delete_confirmation {
        draw_delete_confirmation(f, area);
        return;
    }

    if app.show_modal {
        draw_todo_modal(f, area, app.selected_todo.as_ref().unwrap());
        return;
    }

    // Main table view layout
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Min(1),    // Main table area
            Constraint::Length(3), // Stats area
            Constraint::Length(1), // Shortcuts area
        ])
        .split(area);

    // Futuristic table header
    let header = Row::new(vec![
        "ID", "PRIORITY", "TOPIC", "CONTENT", "CREATED", "STATUS",
    ])
    .style(Style::default().fg(accent).add_modifier(Modifier::BOLD));

    // Table rows with status-based coloring
    let rows = app.todos.iter().map(|todo| {
        Row::new(vec![
            todo.id.to_string().fg(text_primary),
            match todo.priority.to_lowercase().as_str() {
                "high" => todo.priority.clone().fg(Color::Rgb(255, 50, 100)), // Neon red
                "medium" => todo.priority.clone().fg(Color::Rgb(255, 200, 0)), // Amber
                _ => todo.priority.clone().fg(Color::Rgb(50, 255, 100)),      // Neon green
            },
            todo.topic.clone().fg(text_primary),
            todo.text.clone().fg(text_secondary),
            todo.date_added.clone().fg(text_secondary),
            match todo.status.as_str() {
                "Done" | "Completed" => todo.status.clone().fg(Color::Rgb(50, 255, 100)), // Neon green
                "In Progress" => todo.status.clone().fg(Color::Rgb(255, 200, 0)),         // Amber
                "Planned" => todo.status.clone().fg(accent),
                "Backlog" => todo.status.clone().fg(Color::Rgb(255, 50, 100)), // Neon red
                _ => todo.status.clone().fg(text_primary),
            },
        ])
    });

    // Futuristic table styling
    let table = Table::new(
        rows.collect::<Vec<_>>(),
        vec![
            Constraint::Length(5),
            Constraint::Length(12),
            Constraint::Length(15),
            Constraint::Percentage(35),
            Constraint::Length(12),
            Constraint::Length(10),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .title(" TASK MANAGEMENT SYSTEM ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border))
            .style(Style::default().bg(background)),
    )
    .highlight_style(Style::default().bg(highlight).fg(text_primary))
    .row_highlight_style(Style::default().add_modifier(Modifier::BOLD))
    .column_spacing(1);

    f.render_stateful_widget(table, layout[0], &mut app.state);

    // Stats with futuristic styling
    let (completed, in_progress, planned, backlog) = calculate_stats(&app.todos);
    let stats_text = format!(
        "TOTAL: {} | COMPLETED: {} | IN PROGRESS: {} | PLANNED: {} | BACKLOG: {}",
        app.todos.len(),
        completed,
        in_progress,
        planned,
        backlog
    );

    let status_line = Paragraph::new(stats_text)
        .style(Style::default().fg(accent))
        .block(
            Block::default()
                .borders(Borders::TOP)
                .border_style(Style::default().fg(border))
                .style(Style::default().bg(background)),
        );

    f.render_widget(status_line, layout[1]);

    // Shortcuts with consistent styling
    let shortcuts = Paragraph::new(get_shortcuts_text())
        .style(Style::default().fg(text_secondary))
        .block(Block::default().style(Style::default().bg(background)));

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
