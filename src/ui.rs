use crate::App;
use crate::data::Todo;
use ratatui::prelude::Stylize;
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
    if app.show_modal {
        draw_modal(f, area, app.selected_todo.as_ref().unwrap());
    } else {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Min(1), Constraint::Length(3)])
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
                .fg(Color::Magenta)
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
    }
}

pub fn draw_modal(f: &mut Frame, area: Rect, todo: &Todo) {
    // Define a block for the modal with a dark background and magenta border
    let block = Block::default()
        .title("Todo Details")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::DarkGray))
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
            todo.id.to_string().bold().fg(Color::Magenta),
        ]),
        Line::from(""),
        Line::from(vec![
            "Priority: ".into(),
            if todo.priority == "High" {
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
            todo.topic.as_str().bold().fg(Color::Magenta),
        ]),
        Line::from(""),
        Line::from(vec![
            "Status: ".into(),
            todo.status.as_str().bold().fg(Color::Magenta),
        ]),
        Line::from(""),
        Line::from(vec![
            "Date Added: ".into(),
            todo.date_added.as_str().bold().fg(Color::Magenta),
        ]),
        Line::from(""),
        Line::from("Description:"),
        Line::from(""),
        Line::from(todo.text.as_str().bold().fg(Color::Magenta)),
    ];

    // Create a paragraph with the styled text
    let paragraph = Paragraph::new(text)
        .wrap(Wrap { trim: true })
        .block(Block::default());

    f.render_widget(paragraph, inner_area);
}
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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
