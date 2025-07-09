use arguments::models;
use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use data::{Todo, sample_todos};
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
use std::io;

mod arguments;
mod data; // DATABASE STUFF;
mod database;
mod ui; // ALL THE UI STUFF

#[derive(Debug)]
struct App {
    todos: Vec<Todo>,
    state: TableState,
    show_modal: bool,
    selected_todo: Option<Todo>,
}

impl App {
    fn new(todos: Vec<Todo>) -> Self {
        let mut state = TableState::default();
        state.select(Some(0)); // Select first item by default
        Self {
            todos,
            state,
            show_modal: false,
            selected_todo: None,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.todos.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.todos.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn select_current(&mut self) {
        if let Some(index) = self.state.selected() {
            if index < self.todos.len() {
                self.selected_todo = Some(self.todos[index].clone());
                self.show_modal = true;
            }
        }
    }

    fn close_modal(&mut self) {
        self.show_modal = false;
        self.selected_todo = None;
    }
}

fn calculate_stats(todos: &[Todo]) -> (usize, usize, usize, usize) {
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

fn draw_ui(f: &mut Frame, app: &mut App) {
    let area = f.area();

    if app.show_modal {
        draw_modal(f, area, app.selected_todo.as_ref().unwrap());
    } else {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Min(1), Constraint::Length(3)])
            .split(area);

        let header = Row::new(vec!["ID", "Name", "Topic", "Text", "Date Added", "Status"]).style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );

        let rows = app.todos.iter().map(|todo| {
            Row::new(vec![
                todo.id.to_string(),
                todo.name.clone(),
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
        .block(Block::default().title("📝 TODO List").borders(Borders::ALL))
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
            .style(Style::default().fg(Color::Green))
            .block(Block::default().borders(Borders::TOP));

        f.render_widget(status_line, layout[1]);
    }
}

fn draw_modal(f: &mut Frame, area: Rect, todo: &Todo) {
    let block = Block::default()
        .title("Todo Details")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::DarkGray));

    let area = centered_rect(60, 60, area);
    f.render_widget(block, area);

    let inner_area = area.inner(Margin {
        vertical: 2,
        horizontal: 3,
    });

    let text = vec![
        Line::from(vec!["ID: ".into(), todo.id.to_string().bold()]),
        Line::from(""),
        Line::from(vec!["Name: ".into(), todo.name.as_str().bold()]),
        Line::from(""),
        Line::from(vec!["Topic: ".into(), todo.topic.as_str().bold()]),
        Line::from(""),
        Line::from(vec!["Status: ".into(), todo.status.as_str().bold()]),
        Line::from(""),
        Line::from(vec!["Date Added: ".into(), todo.date_added.as_str().bold()]),
        Line::from(""),
        Line::from("Description:"),
        Line::from(""),
        Line::from(todo.text.as_str()),
    ];

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

fn main() -> Result<(), io::Error> {
    let cli = models::Cli::parse();

    if cli.list {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let todos = sample_todos();
        let mut app = App::new(todos);

        loop {
            terminal.draw(|f| draw_ui(f, &mut app))?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Down | KeyCode::Char('j') => app.next(),
                    KeyCode::Up | KeyCode::Char('k') => app.previous(),
                    KeyCode::Enter => {
                        if app.show_modal {
                            app.close_modal();
                        } else {
                            app.select_current();
                        }
                    }
                    KeyCode::Esc => {
                        if app.show_modal {
                            app.close_modal();
                        }
                    }
                    _ => {}
                }
            }
        }

        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;
    }

    // DELETE TODO WITH PROVIDED ID
    if let Some(id) = cli.delete {
        match arguments::delete_todo::remove_todo(id) {
            Ok(_) => println!("Todo deleted successfully!"),
            Err(e) => eprintln!("Error deleting todo: {}", e),
        }
    }

    // PASS THE ARGUMENTS
    if let Some(words) = cli.add {
        let text = words.join(" ");

        match arguments::add_todo::add_todo(text, cli.topic) {
            Ok(_) => println!("Todo added successfully!"),
            Err(e) => eprintln!("Error adding todo: {}", e),
        }
    } else if cli.print {
        arguments::print::print_todos();
    }

    Ok(())
}
