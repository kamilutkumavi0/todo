use todo_cli::{parse_it, clap_structure::{Status, Subs}};
use crate::basics::{print_todo, add_todo, delete_todo, update_todo, init_todo};
    use todo_structure::{todo::TodoStatus};
pub fn user_input(){
    let parsed_arg = parse_it();
	let path = parsed_arg.path;
    match parsed_arg.sub_command{
        Some(Subs::Del(arg)) => {

            delete_todo(path, arg.name);
        },
        Some(Subs::Add(arg)) => {
            let status = match arg.status{
                Status::NotStarted => TodoStatus::NotStarted,
                Status::Continue => TodoStatus::Continue,
                Status::Done => TodoStatus::Done,
            };
            add_todo(path, arg.name, arg.description, arg.start_date, arg.finish_date, status);
        },
        Some(Subs::Update(arg)) => {
            let status = match arg.status{
                Status::NotStarted => TodoStatus::NotStarted,
                Status::Continue => TodoStatus::Continue,
                Status::Done => TodoStatus::Done,
            };
            update_todo(path, arg.name, Some(status));
        },
        Some(Subs::Init) => init_todo(),
        None => print_todo(path),
    } 
}
