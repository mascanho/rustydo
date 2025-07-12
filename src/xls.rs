use crate::database::DBtodo;
use xlsxwriter::*;

pub fn export_todos() -> Result<(), XlsxError> {
    // Initialize database and get todos
    let db = DBtodo::new().expect("Failed to initialize database");
    let todos = db.get_todos().expect("Failed to get todos");

    // Create workbook
    let mut workbook = Workbook::new("todos.xlsx")?;
    let mut worksheet = workbook.add_worksheet(None)?;

    // Write headers
    let headers = [
        "ID",
        "PRIORITY",
        "TOPIC",
        "TODO",
        "DESCRIPTION",
        "CREATED",
        "DUE DATE",
        "STATUS",
        "OWNER",
    ];

    for (col_num, header) in headers.iter().enumerate() {
        worksheet.write_string(0, col_num as u16, header, None)?;
    }

    // Write data
    for (row_num, todo) in todos.iter().enumerate() {
        let row = row_num as u32 + 1;

        // Handle potential Option fields with unwrap_or_default()
        worksheet
            .write_number(row, 0, todo.id as f64, None)
            .expect("Failed to write ID");
        worksheet.write_string(row, 1, &todo.priority.to_string(), None)?;
        worksheet.write_string(row, 2, &todo.topic, None)?;
        worksheet.write_string(row, 3, &todo.text, None)?;
        worksheet.write_string(row, 4, &todo.desc, None)?;
        worksheet.write_string(row, 5, &todo.date_added.to_string(), None)?;
        worksheet.write_string(row, 6, &todo.due.to_string(), None)?;
        worksheet.write_string(row, 7, &todo.status.to_string(), None)?;
        worksheet.write_string(row, 8, &todo.owner, None)?;
    }

    workbook.close()?;
    println!("");
    println!("🤖 Todos exported to todos.xlsx");
    println!("");
    Ok(())
}
