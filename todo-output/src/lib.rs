use todo_structure::todo::{Todo, TodoStatus};
use colored::Colorize;

pub fn todo_print(todo: Todo){
    match todo.status{
        Some(TodoStatus::NotStarted) => print!("{} ", "✖".red()),
        Some(TodoStatus::Continue) => print!("{} ", "⧗".yellow()),
        Some(TodoStatus::Done) => print!("{} ", "✔".green()),
        None => print!("  "),
    }
    print!("{} ", todo.name.bold());

    let description = match todo.description{
                            Some(a) => a,
                            None => String::new(),
                    };
    print!("{} ", description.bright_blue());
    match todo.start_date{
        Some(date) => print!("{} -> ", date),
        None => print!(" "),
    };
    match todo.finish_date{
        Some(date) => print!("{} ", date),
        None => print!(" "),
    };
    println!("");
}

pub fn todo_vec_print(todo_vec: Vec<Todo>){
    for todo in todo_vec{
        todo_print(todo);
    }
}
