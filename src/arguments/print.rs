use crate::data;

pub fn print_todos() {
    let todos = data::sample_todos();

    let todo_total = todos.len();
    println!("");
    println!("Total todos: {}", todo_total);

    let mut todo_count = 0;

    for todo in todos {
        todo_count += 1;
        println!("---------------");
        println!("Todo #{}", todo_count);
        println!("ID: {}", todo.id);
        println!("Priority: {}", todo.priority);
        println!("Topic: {}", todo.topic);
        println!("Text: {}", todo.text);
        println!("Description: {}", todo.desc);
        println!("Date Added: {}", todo.date_added);
        println!("Due: {}", todo.due);
        println!("Status: {}", todo.status);
        println!("Owner: {}", todo.owner);
        println!("---------------");
    }
}
