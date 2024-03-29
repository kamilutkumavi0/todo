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
    
    pub fn delete_todo(){
        
    }

    pub fn update_todo(){
        
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
}
