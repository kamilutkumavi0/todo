use todo_structure::{todo::{Todo, TodoStatus},date::TodoDate};
pub fn todo_print(todo: Todo){
    print!("{} ", todo.name);
    print!("{} ", todo.description.unwrap_or(String::new()));
    print!("{} ", todo.start_date.unwrap());
    print!("{} ", todo.finish_date.unwrap());
    match todo.status.unwrap(){
        TodoStatus::NotStarted => print!("-"),
        TodoStatus::Continue => print!("!"),
        TodoStatus::Done => print!("+"),
    }
    println!("");
}

pub fn todo_vec_print(todo_vec: Vec<Todo>){
    for todo in todo_vec{
        todo_print(todo);
    }
}
