#[derive(Debug, Clone)]
pub struct Todo {
    pub id: usize,
    pub name: String,
    pub topic: String,
    pub text: String,
    pub date_added: String,
    pub status: String,
}

pub fn sample_todos() -> Vec<Todo> {
    vec![
        Todo {
            id: 1,
            name: "Learn Ratatui".into(),
            topic: "Rust UI".into(),
            text:
                "Understand rendering with tui and how to build interactive terminal applications"
                    .into(),
            date_added: "2025-07-08".into(),
            status: "In Progress".into(),
        },
        Todo {
            id: 2,
            name: "Add GPT Support".into(),
            topic: "AI".into(),
            text: "Use GPT to auto-tag tasks and generate descriptions based on minimal input"
                .into(),
            date_added: "2025-07-07".into(),
            status: "Planned".into(),
        },
        Todo {
            id: 3,
            name: "Implement Search".into(),
            topic: "Features".into(),
            text: "Add search functionality to filter todos by name, topic or status".into(),
            date_added: "2025-07-06".into(),
            status: "Backlog".into(),
        },
        Todo {
            id: 4,
            name: "Improve UI".into(),
            topic: "Design".into(),
            text: "Add colors and better formatting to make the UI more appealing".into(),
            date_added: "2025-07-05".into(),
            status: "Completed".into(),
        },
    ]
}
