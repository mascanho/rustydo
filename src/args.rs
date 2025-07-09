pub fn print_args() {
    println!(
        "
    Usage: rustydo [options]

    Options:
    -h, --help          Print this help message
    -a, --add           Add a new todo
    -r, --remove        Remove a todo by ID
    -u, --update        Update a todo by ID
    -d, --done          Mark a todo as done by ID
    -p, --print         Print all todos"
    )
}
