use crate::database::DBtodo;

#[derive(Debug, Clone)]
pub struct Todo {
    pub id: usize,
    pub priority: String,
    pub topic: String,
    pub text: String,
    pub date_added: String,
    pub status: String,
}

pub fn sample_todos() -> Vec<Todo> {
    let db = DBtodo::new().unwrap();

    let todos = db.get_todos().unwrap().iter().cloned().collect();
    todos
}
