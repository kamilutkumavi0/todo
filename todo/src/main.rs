use file_format::{read::read_todo, write::write_todo};
use todo_structure::todo::Todo;
use todo::splitter::split_line;
use todo_output::todo_vec_print;
fn main() {
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
