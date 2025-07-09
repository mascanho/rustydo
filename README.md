# Rustydo

![Rustydo Logo](https://raw.githubusercontent.com/placeholder/rustydo/main/logo.png)

Rustydo is a powerful and intuitive command-line interface (CLI) todo application built with Rust. It allows you to manage your tasks efficiently directly from your terminal, offering both a interactive terminal user interface (TUI) and a set of commands for quick operations.

## Features

- **Interactive TUI**: A full-featured terminal UI for managing your todos with keyboard navigation.
- **Add Todos**: Easily add new tasks with optional topics and priorities.
- **Delete Todos**: Remove tasks by their ID.
- **Update Todos**: Mark tasks as done or change their status.
- **Clear All Todos**: Clear all your tasks with a single command.
- **List Todos**: View all your tasks in a clear, formatted list.
- **Persistent Storage**: Your todos are saved locally using SQLite.

## Installation

To install Rustydo, you need to have Rust and Cargo installed on your system. If you don't have them, you can install them by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

Once Rust is installed, you can clone this repository and build the project:

```bash
git clone https://github.com/your-username/rustydo.git
cd rustydo
cargo install --path .
```

This will install the `rustydo` executable to your Cargo bin directory, usually `~/.cargo/bin`.

## Usage

### Interactive Terminal UI

To launch the interactive TUI, run:

```bash
rustydo --list
```

In the TUI:
- Use `Up` and `Down` arrow keys or `k` and `j` to navigate.
- Press `Enter` to view details of a selected todo.
- Press `Esc` to close the details modal.
- Press `q` to quit the application.

### Command-Line Operations

Here are the available command-line options:

#### Add a new todo

```bash
rustydo --add "Buy groceries" --topic "Shopping" --priority "High"
```

- `--add <TEXT>`: The description of your todo. (Required)
- `--topic <TOPIC>`: Optional topic for the todo.
- `--priority <PRIORITY>`: Optional priority (e.g., "High", "Medium", "Low").

#### Delete a todo

```bash
rustydo --remove <ID>
```

- `--remove <ID>`: The ID of the todo to delete.

#### Update a todo's status

```bash
rustydo --update-id <ID> --status "Done"
```

- `--update-id <ID>`: The ID of the todo to update.
- `--status <STATUS>`: The new status for the todo (e.g., "Done", "Pending", "In Progress").

#### Mark a todo as done (shortcut)

```bash
rustydo --done <ID>
```

- `--done <ID>`: The ID of the todo to mark as "Done".

#### Clear all todos

```bash
rustydo --clear
```

#### Print all todos to the console

```bash
rustydo --print
```

#### Get help

```bash
rustydo --help
```

## Technologies Used

- [Rust](https://www.rust-lang.org/)
- [Ratatui](https://ratatui.rs/) (for the TUI)
- [Crossterm](https://docs.rs/crossterm/latest/crossterm/) (for terminal manipulation)
- [Clap](https://docs.rs/clap/latest/clap/) (for command-line argument parsing)
- [Rusqlite](https://docs.rs/rusqlite/latest/rusqlite/) (for SQLite database interaction)
- [Serde](https://serde.rs/) (for serialization/deserialization)
- [Chrono](https://docs.rs/chrono/latest/chrono/) (for date and time handling)

## Contributing

Contributions are welcome! Please feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details. (Note: A LICENSE file is not yet present in the repository. Please create one if you intend to use this license.)