pub mod user_input;

pub fn split_line(text: String) -> Vec<String>{
	let mut output: Vec<String> = Vec::new();
   	let mut flag = false;
   	let mut temp: String = String::new();
   	for i in text.chars(){
       	if i == ']' && flag{
           	flag = false;
           	output.push(temp);
           	temp = String::new();
       	}
       	else if i == '[' && !flag{
           	flag = true;
       	}
       	else if flag {
           	temp.push(i);
       	}
   	}
 	output
}


pub mod basics{
    use file_format::{read::read_todo, write::write_todo};
    use todo_output::todo_vec_print;
    use todo_structure::{todo::{Todo, TodoStatus}, date::TodoDate};
    use crate::split_line;

    fn file_to_todo(path: &String)->Vec<Todo>{
        let text = read_todo(path.to_string());
        let mut todo_vec: Vec<Todo> = Vec::new();
        let o = split_line(text);
        for i in o{
            let todo = Todo::from(i);
            todo_vec.push(todo);
        }
        todo_vec
    }
    
    pub fn delete_todo(path: String, name: String){
        let mut todo_vec = file_to_todo(&path);
        for (i, todo) in todo_vec.clone().into_iter().enumerate(){
            if todo.name == name{
                todo_vec.remove(i);
                break;
            }
        }
        write_todo(path, todo_vec);
    }

    pub fn update_todo(path: String, name: String, status: Option<TodoStatus>){
        let mut todo_vec = file_to_todo(&path);
        for (i, todo) in todo_vec.clone().into_iter().enumerate(){
            if todo.name == name{
                let mut update_todo = todo_vec.remove(i);
                update_todo.status = status;
                todo_vec.insert(i, update_todo);
                break;
            }
        }
        write_todo(path, todo_vec);
    }

    pub fn add_todo(path: String, name: String, description: Option<String>, start_date: Option<String>, finish_date: Option<String>, status: TodoStatus){
        let mut todo_vec = file_to_todo(&path);
        let start_date = match start_date{
            Some(i) => Some(TodoDate::from_string(i).unwrap()),
            None => None,
        };
        let finish_date = match finish_date{
            Some(i) => Some(TodoDate::from_string(i).unwrap()),
            None => None,
        };
        let new = Todo::new(name, description, start_date, finish_date, Some(status)).unwrap();
        todo_vec.push(new);
        write_todo(path, todo_vec);
    }

    pub fn print_todo(path: String){
        let todo_vec = file_to_todo(&path);
        todo_vec_print(todo_vec.clone());
    }

    pub fn init_todo(){
        let new: Vec<Todo> = Vec::new();
        write_todo(String::from("./todo.td"), new);
    }
}
