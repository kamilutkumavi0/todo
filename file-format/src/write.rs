use std::fs::write;
use todo_structure::todo::{Todo, TodoStatus};

pub fn write_todo(todo_vec: Vec<Todo>){
    let mut todo_string = String::new();
    for todo in todo_vec{
        todo_string.push('[');
        todo_string.push('"');
        todo_string.push_str(&todo.name);
        todo_string.push('"');
        
        todo_string.push(',');
        
        todo_string.push('"');
        match todo.description{
            Some(s) => todo_string.push_str(&s),
            None => todo_string.push(' '),
        }
        todo_string.push('"');
        
        todo_string.push(',');
        
        todo_string.push('"');
        match todo.start_date{
            Some(d) => {
                let d_str = format!("{}", d);
                todo_string.push_str(&d_str);
            }
            None => todo_string.push(' '),
        }
        todo_string.push('"');

        todo_string.push(',');
        
        todo_string.push('"');
        match todo.finish_date{
            Some(d) => {
                let d_str = format!("{}", d);
                todo_string.push_str(&d_str);
            }
            None => todo_string.push(' '),
        }
        todo_string.push('"');

        todo_string.push(',');
        
        todo_string.push('"');
        match todo.status{
            Some(TodoStatus::NotStarted) => todo_string.push_str("NotStarted"),
            Some(TodoStatus::Continue) => todo_string.push_str("Continue"),
            Some(TodoStatus::Done) => todo_string.push_str("Done"),
            None => todo_string.push(' '),
        }
        // pub status: Option<TodoStatus>,
        todo_string.push('"');
        todo_string.push(']');
        todo_string.push('\n');
    }
    let _ = write("./deneme.todo", todo_string.as_bytes());
}
