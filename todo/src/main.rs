use file_format::{read::read_todo, write::write_todo};
use todo_structure::todo::Todo;
use todo::{split_line, user_input::user_input};
use todo_output::todo_vec_print;


fn main() {
    user_input();
    let text = read_todo();
    let mut todo_vec: Vec<Todo> = Vec::new();
    let o = split_line(text);
    for i in o{
        let todo = Todo::from(i);
        todo_vec.push(todo);
    }
    todo_vec_print(todo_vec.clone());
    write_todo(todo_vec);
}
