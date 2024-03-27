//! Create a todo structure with this library
use crate::date::TodoDate;

fn split(text: String) -> Vec<String>{
    let mut output: Vec<String> = Vec::new();
    let mut flag = false;
    let mut temp: String = String::new();
    for i in text.chars(){
        if i == '"' && flag{
            flag = false;
            output.push(temp);
            temp = String::new();
        }
        else if i == '"' && !flag{
            flag = true;
        }
        else if flag {
            temp.push(i);
        }
    }
    output
}

/// TodoStatus is a enumaration for the understand the jobs status:
/// * NotStarted
/// * Continue
/// * Done
///
/// can created like in example
/// ## Example
/// ```rust
/// # use todo_structure::todo::TodoStatus; 
/// let todo_status = TodoStatus::Done;
/// println!("{:?}", todo_status);
/// ```
#[derive(PartialEq, Debug)]
pub enum TodoStatus{
    NotStarted,
    Continue,
    Done,
}

/// TodoError is a enumarate of the Todo structure's error
#[derive(PartialEq, Debug)]
pub enum TodoError{
    WrongOrderDate,
    NoName,
}

/// Todo is the structure of the todo can create structure for easy to read in the code
/// this structure is using in the file reader and writer format.
///```rust
/// # use todo_structure::todo::Todo;
/// # use todo_structure::todo::TodoStatus;
/// use todo_structure::date::TodoDate;
/// Todo{ name: "name".to_string(),  description: Some("description".to_string()), start_date: Some(TodoDate::new( 1, 4, 1999).unwrap()), finish_date: Some(TodoDate::new(1, 4, 2000).unwrap()), status: Some(TodoStatus::Done)};
///```
#[derive(PartialEq, Debug)]
pub struct Todo{
    pub name: String,
    pub description: Option<String>,
    pub start_date: Option<TodoDate>,
    pub finish_date: Option<TodoDate>,
    pub status: Option<TodoStatus>,
}

impl Todo{
    /// Creates new Todo
    ///```rust
    /// # use todo_structure::todo::Todo;
    /// # use todo_structure::todo::TodoStatus;
    /// use todo_structure::date::TodoDate;
    /// Todo::new("name".to_string(), Some("description".to_string()), Some(TodoDate::new( 1, 4, 1999).unwrap()), Some(TodoDate::new(1, 4, 2000).unwrap()), Some(TodoStatus::Done));
    ///```
    pub fn new(name: String, description: Option<String>, start_date: Option<TodoDate>, finish_date: Option<TodoDate>, status: Option<TodoStatus>) -> Result<Self,TodoError> {
        if name == ""{
            return Err(TodoError::NoName);
        }
        if start_date != None && finish_date != None{
            if finish_date < start_date{
                return Err(TodoError::WrongOrderDate);
            }
        }
        Ok(Self{name, description, start_date, finish_date, status})
    }
    pub fn from(text: String) -> Self{
        let element_vec: Vec<String> = split(text);
        Self{
            name: element_vec[0].clone(),
            description: Some(element_vec[1].clone()),
            start_date: Some(TodoDate::from_string(element_vec[2].clone()).unwrap()),
            finish_date: Some(TodoDate::from_string(element_vec[3].clone()).unwrap()),
            status: match element_vec[4].clone().as_str() {
                "Done" => Some(TodoStatus::Done),
                "NotStarted" => Some(TodoStatus::NotStarted),
                "Continue" => Some(TodoStatus::Continue) ,
                _ => None,
            },
        }
    }
}
