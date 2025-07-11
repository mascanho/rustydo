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

mod ai; // LLMS stuff
mod args; // Print all the args available in the App so it does not clutter the main.rs
mod arguments;
mod colors;
mod configs;
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

    // CHANGE TODO STATUS
    fn change_todo_status(
        &mut self,
        id: i32,
        status: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Validate selection exists
        let selected = self.state.selected().ok_or("No todo selected")?;

        // Validate selection is within bounds
        if selected >= self.todos.len() {
            return Err("Invalid selection".into());
        }

        // Update database
        let db = database::DBtodo::new()?;
        db.update_todo(id, Some(status.clone()))?;

        // Update local state
        self.todos[selected].status = status;

        // Maintain selection position
        if !self.todos.is_empty() {
            let new_selection = selected.min(self.todos.len().saturating_sub(1));
            self.state.select(Some(new_selection));
        }

        Ok(())
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

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    // Create the configs
    let _ = configs::AppConfigs::create_default_config();

    let cli = Cli::parse();

    // Check if no arguments were provided
    let no_args_provided = std::env::args().count() == 1;

    // Terminal UI mode (default when no args provided or when --list is explicitly set)
    if cli.list || no_args_provided {
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
                    // CHANGE TODO STATUS
                    KeyCode::Char('p') => {
                        if let Some(selected) = app.state.selected() {
                            if selected < app.todos.len() {
                                let id = app.todos[selected].id;
                                let status = "Pending".to_string();
                                if let Err(e) = app.change_todo_status(id as i32, status) {
                                    eprintln!("Error updating todo status: {}", e);
                                }
                            }
                        }
                    }

                    KeyCode::Char('f') => {
                        if let Some(selected) = app.state.selected() {
                            if selected < app.todos.len() {
                                let id = app.todos[selected].id;
                                let status = "Done".to_string();
                                if let Err(e) = app.change_todo_status(id as i32, status) {
                                    eprintln!("Error updating todo status: {}", e);
                                }
                            }
                        }
                    }

                    KeyCode::Char('o') => {
                        if let Some(selected) = app.state.selected() {
                            if selected < app.todos.len() {
                                let id = app.todos[selected].id;
                                let status = "Ongoing".to_string();
                                if let Err(e) = app.change_todo_status(id as i32, status) {
                                    eprintln!("Error updating todo status: {}", e);
                                }
                            }
                        }
                    }

                    // Delete todo
                    KeyCode::Char('d') => {
                        if !app.todos.is_empty() {
                            app.show_delete_confirmation = true;
                        }
                    }

                    // Handle delete confirmation
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
    // PROMPT GEMINI
    else if let Some(prompt) = cli.prompt {
        match ai::ask_gemini(prompt).await {
            Ok(response) => println!("{}", response),
            Err(e) => eprintln!(
                "Error: {}. Please set an API key first using the -k flag.",
                e
            ),
        }
    }
    // Pass the API key
    else if let Some(key) = cli.apikey {
        let db = database::DBtodo::new().unwrap();
        db.set_api_credentials(Some(key)).unwrap_or_else(|e| {
            eprintln!("Error setting API credentials: {}", e);
        })
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
        if let Err(e) = arguments::update_todo::update_todo(id, status) {
            eprintln!("Error updating todo: {}", e);
        }
    }
    // UPDATE USING SHORT FORMAT
    else if let Some(id) = cli.done {
        if let Err(e) = arguments::update_todo::update_todo(id, "Done".to_string()) {
            eprintln!("Error updating todo: {}", e);
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

    Ok(())
}
