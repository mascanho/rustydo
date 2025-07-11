<div align="center">
  <img src="https://github.com/mascanho/rustydo/blob/master/src/images/logo.png" alt="Rustydo Logo" width="200" style="border-radius: 10px; box-shadow: 0 4px 8px rgba(0,0,0,0.1); margin: 20px 0;">
  <h1>RustyDo</h1>
</div>

**RustyDo** is a powerful and intuitive command-line (CLI) todo application built with Rust, supercharged with AI capabilities. It allows you to manage your tasks efficiently directly from your terminal, offering both an interactive terminal user interface (TUI) and a comprehensive set of commands for quick, command-line operations.

## ✨ Features

- **Interactive TUI**: A full-featured terminal UI to manage your todos with keyboard navigation.
- **AI-Powered Suggestions**: Leverage the power of Google's Gemini to get task suggestions based on your prompts.
- **Comprehensive Task Management**: Add, delete, and update tasks with details like topics, priorities, owners, and due dates.
- **Flexible Commands**: Use a wide range of flags to manage your todos without ever leaving the command line.
- **Persistent Storage**: Your todos are saved locally in a SQLite database, ensuring your data is always safe.
- **Configuration File**: Automatically creates a configuration file to manage settings like API keys.

## 🚀 Installation

To install RustyDo, you need to have Rust and Cargo installed. If you don't, follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

Once Rust is set up, clone the repository and install the application:

```bash
git clone https://github.com/mascanho/rustydo.git
cd rustydo
cargo install --path .
```

This will install the `rustydo` executable in your Cargo bin directory (usually `~/.cargo/bin`), making it available from anywhere in your terminal.

## ⚙️ Configuration

The first time you run RustyDo, it will automatically create a `config.toml` file in your system's configuration directory. To use the AI features, you need to add your Google Gemini API key to this file.

1.  **Get your API key**: Obtain your API key from [Google AI Studio](https://aistudio.google.com/app/apikey).
2.  **Set the key**: You can set the API key using the following command:
    ```bash
    rustydo --apikey YOUR_API_KEY
    ```
    This will securely save your key to the configuration file.

## 💻 Usage

### Interactive Terminal UI (TUI)

Launch the interactive TUI with the `--list` or `-l` flag:

```bash
rustydo --list
```

**Controls:**
-   **Navigate**: `Up`/`Down` arrow keys or `k`/`j`.
-   **View Details**: `Enter` to open the details modal for the selected todo.
-   **Change Status**:
    -   `p`: Mark as "Pending".
    -   `o`: Mark as "Ongoing".
    -   `f`: Mark as "Done".
-   **Delete Todo**: `d` to open a confirmation dialog, then `y` to confirm or `n` to cancel.
-   **Close Modals**: `Esc` to close any open modal.
-   **Quit**: `q` to exit the application.

### Command-Line Operations

Here are the available command-line options:

#### 🤖 AI Commands

**Get AI-powered task suggestions:**
```bash
rustydo --prompt "plan a marketing campaign for a new product launch"
```

**Set your Gemini API key:**
```bash
rustydo --apikey YOUR_API_KEY
```

#### ✅ Todo Management

**Add a new todo:**
```bash
rustydo --add "Deploy the new feature to production" --desc "Ensure all tests pass" --topic "DevOps" --priority "High" --owner "Alex" --due "2024-12-31"
```
- `--add <TEXT>`: The description of the todo. (Required)
- `--desc <TEXT>`: A more detailed description. (Optional)
- `--topic <TOPIC>`: A topic for categorization. (Optional)
- `--priority <PRIORITY>`: Priority level (e.g., "High", "Medium", "Low"). (Optional)
- `--owner <OWNER>`: The person responsible for the task. (Optional)
- `--due <DATE>`: A due date for the task. (Optional)

**Delete a todo:**
```bash
rustydo --delete <ID>
```

**Update a todo's status:**
```bash
rustydo --update-id <ID> --status "Ongoing"
```

**Mark a todo as "Done" (shortcut):**
```bash
rustydo --done <ID>
```

**Clear all todos:**
```bash
rustydo --clear
```

#### utility
**Print all todos to the console:**
```bash
rustydo --print
```

**Show available arguments:**
```bash
rustydo --show
```

**Get help:**
```bash
rustydo --help
```

## 🛠️ Technologies Used

- [Rust](https://www.rust-lang.org/)
- [Ratatui](https://ratatui.rs/) (for the TUI)
- [Tokio](https://tokio.rs/) (for asynchronous operations)
- [Reqwest](https://docs.rs/reqwest/latest/reqwest/) (for HTTP requests to the Gemini API)
- [Crossterm](https://docs.rs/crossterm/latest/crossterm/) (for terminal manipulation)
- [Clap](https://docs.rs/clap/latest/clap/) (for argument parsing)
- [Rusqlite](https://docs.rs/rusqlite/latest/rusqlite/) (for SQLite database)
- [Serde](https://serde.rs/) (for serialization/deserialization)
- [TOML](https://docs.rs/toml/latest/toml/) (for configuration file parsing)
- [Chrono](https://docs.rs/chrono/latest/chrono/) (for date and time)
- [Anyhow](https://docs.rs/anyhow/latest/anyhow/) (for error handling)
- [Directories](https://docs.rs/directories/latest/directories/) (for finding config paths)

## 🙌 Contributing

Contributions are welcome! If you have ideas for new features or find a bug, please open an issue or submit a pull request.

## 📄 License

This project is licensed under the MIT License.