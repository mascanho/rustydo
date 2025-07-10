use arguments::{
    delete_todo,
    models::{self, Cli, Todo},
};
use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use data::sample_todos;
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
use ui::{calculate_stats, draw_ui};

mod args; // Print all the args available in the App so it does not clutter the main.rs
mod arguments;
mod colors;
mod data; // DATABASE STUFF;
mod database;
mod modals; // All the modals logic
mod ui; // ALL THE UI STUFF

#[derive(Debug)]
pub struct App {
    pub todos: Vec<Todo>,
    pub state: TableState,
    pub show_modal: bool,
    pub selected_todo: Option<Todo>,
    pub show_delete_confirmation: bool,
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
            show_delete_confirmation: false,
        }
    }

    fn delete_current_todo(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(selected) = self.state.selected() {
            if selected < self.todos.len() {
                let id = self.todos[selected].id;
                let db = database::DBtodo::new()?;
                db.delete_todo(id as i32)?;

                // Update local state
                self.todos.remove(selected);

                // Adjust selection
                if !self.todos.is_empty() {
                    self.state.select(Some(selected.min(self.todos.len() - 1)));
                } else {
                    self.state.select(None);
                }
            }
        }
        Ok(())
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

fn main() -> Result<(), io::Error> {
    let cli = Cli::parse();

    // Terminal UI mode
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
                    // Delete todo
                    KeyCode::Char('d') => {
                        if !app.todos.is_empty() {
                            app.show_delete_confirmation = true;
                        }
                    }

                    // Handle confirmation
                    KeyCode::Char('y') if app.show_delete_confirmation => {
                        if let Err(e) = app.delete_current_todo() {
                            eprintln!("Error deleting todo: {}", e);
                        }
                        app.show_delete_confirmation = false;
                    }

                    KeyCode::Char('n') if app.show_delete_confirmation => {
                        app.show_delete_confirmation = false;
                    }
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
    // Add new todo
    else if let Some(words) = cli.add {
        let text = words.join(" ");
        let desc = cli.desc.map(|desc| desc.join(" "));
        match arguments::add_todo::add_todo(text, cli.topic, cli.priority, cli.owner, cli.due, desc)
        {
            Ok(_) => println!("✅ Todo added successfully!"),
            Err(e) => eprintln!("Error adding todo: {}", e),
        }
    }
    // Delete todo
    else if let Some(id) = cli.delete {
        match arguments::delete_todo::remove_todo(id) {
            Ok(_) => println!("✅ Todo deleted successfully!"),
            Err(e) => eprintln!("Error deleting todo: {}", e),
        }
    }
    // Update todo status
    else if let (Some(id), Some(status)) = (cli.update_id, cli.status) {
        match arguments::update_todo::update_todo(id, status) {
            Ok(_) => println!("✅ Todo updated successfully!"),
            Err(e) => eprintln!("Error updating todo: {}", e),
        }
    }
    // UPDATE USING SHORT FORMAT
    else if let Some(id) = cli.done {
        match arguments::update_todo::update_todo(id, "Done".to_string()) {
            Ok(_) => println!("Todo updated successfully!"),
            Err(e) => eprintln!("Error updating todo: {}", e),
        }
    }
    // Clear all todos
    else if cli.clear {
        match arguments::delete_todo::clear_todos() {
            Ok(_) => println!("Todos deleted successfully!"),
            Err(e) => eprintln!("Error deleting todos: {}", e),
        }
    }
    // Print todos
    else if cli.print {
        arguments::print::print_todos();
    }
    // Print args
    else if cli.show {
        args::print_args();
    }
    // Default behavior when no arguments are provided
    else {
        println!("Welcome to RustyDo! Use the -h or --help argument to see available options.");
    }

    Ok(())
}
