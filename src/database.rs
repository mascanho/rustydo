use std::error::Error;

use directories::BaseDirs;
use rusqlite::{Connection, Result, params};

use crate::data::Todo;

pub struct ConfigDir {
    pub config_dir: String,
}

pub struct DBtodo {
    pub connection: rusqlite::Connection,
}

impl ConfigDir {
    pub fn new() -> ConfigDir {
        let base_dirs = BaseDirs::new().unwrap();
        let config_dir = base_dirs.config_dir().join("rustdo");
        ConfigDir {
            config_dir: config_dir.to_str().unwrap().to_string(),
        }
    }
}

impl DBtodo {
    pub fn new() -> Result<DBtodo, Box<dyn Error>> {
        let config_dir = ConfigDir::new();
        let folder = config_dir.config_dir;

        // Check if the folder path exists and is a file
        if std::path::Path::new(&folder).is_file() {
            return Err(format!("Error: Expected a directory at '{}', but found a file. Please remove or rename the file.", folder).into());
        }

        // Create directory if it doesn't exist
        std::fs::create_dir_all(&folder)?;

        // Create the path to the database file
        let db_path = std::path::Path::new(&folder).join("todos.db");
        println!("Database path: {}", db_path.display());

        // Check if db_path exists and is a directory
        if db_path.exists() && db_path.is_dir() {
            return Err(format!("Error: Expected a file at '{}', but found a directory. Please remove or rename the directory.", db_path.display()).into());
        }

        // Open or create the database file
        let connection = Connection::open(&db_path)?;

        // Initialize the table (if it doesn't exist)
        connection.execute(
            "CREATE TABLE IF NOT EXISTS todos (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                topic TEXT,
                text TEXT,
                date_added TEXT NOT NULL,
                status TEXT NOT NULL
            )",
            [],
        )?;

        Ok(DBtodo { connection })
    }

    /// Adds a new todo to the database (better than standalone function)
    pub fn add_todo(&self, todo: &Todo) -> Result<(), Box<dyn Error>> {
        self.connection.execute(
            "INSERT INTO todos (name, topic, text, date_added, status) 
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                &todo.priority,
                &todo.topic,
                &todo.text,
                &todo.date_added,
                &todo.status,
            ],
        )?;
        println!("✅ Todo added successfully!");
        Ok(())
    }

    // DELETE TODO BASED ON ID
    pub fn delete_todo(&self, id: i32) -> Result<(), Box<dyn Error>> {
        let changes = self
            .connection
            .execute("DELETE FROM todos WHERE id = ?", params![id])?;

        if changes > 0 {
            println!("✅ Todo deleted successfully!");
        } else {
            println!("❌ No todo found with id: {}", id);
        }

        Ok(())
    }

    // SHOW ALL THE TODOS
    pub fn get_todos(&self) -> Result<Vec<Todo>, Box<dyn Error>> {
        let mut stmt = self.connection.prepare("SELECT * FROM todos")?;
        let todos_iter = stmt.query_map(params![], |row| {
            Ok(Todo {
                id: row.get(0)?,
                priority: row.get(1)?,
                topic: row.get(2)?,
                text: row.get(3)?,
                date_added: row.get(4)?,
                status: row.get(5)?,
            })
        })?;

        let mut todos: Vec<Todo> = Vec::new();
        for todo in todos_iter {
            todos.push(todo.unwrap());
        }
        Ok(todos)
    }

    // UPDATE TODO STATUS
    pub fn update_todo(&self, id: i32, status: Option<String>) -> Result<(), Box<dyn Error>> {
        let changes = self.connection.execute(
            "UPDATE todos SET status = ? WHERE id = ?",
            params![status, id],
        )?;
        if changes > 0 {
            println!("✅ Todo updated successfully!");
        } else {
            println!("❌ No todo found with id: {}", id);
        }
        Ok(())
    }

    // CLEAR ALL TODOS FROM DB
    pub fn clear_all_todos(&self) -> Result<(), Box<dyn Error>> {
        let changes = self.connection.execute("DELETE FROM todos", params![])?;
        if changes > 0 {
            println!("✅ All todos cleared successfully!");
        } else {
            println!("❌ No todos found.");
        }
        Ok(())
    }
}
