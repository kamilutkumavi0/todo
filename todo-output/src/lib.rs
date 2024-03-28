use todo_structure::{todo::{Todo, TodoStatus},date::TodoDate};
pub fn todo_print(todo: Todo){
    match todo.status{
        Some(TodoStatus::NotStarted) => print!("- "),
        Some(TodoStatus::Continue) => print!("! "),
        Some(TodoStatus::Done) => print!("+ "),
        None => print!("  "),
    }
    print!("{} ", todo.name);

    let description = match todo.description{
                            Some(a) => a,
                            None => String::new(),
                    };
    print!("{} ", description);
    match todo.start_date{
        Some(date) => print!("{} ", date),
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
