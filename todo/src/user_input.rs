use todo_cli::{parse_it, clap_structure::{Status, AddArgs, DelArgs, UpdateArgs, Subs, Todo}};
use crate::basics::{print_todo, add_todo};
    use todo_structure::{todo::TodoStatus};
pub fn user_input(){
    let parsed_arg = parse_it();
	let path = parsed_arg.path;
    match parsed_arg.sub_command{
        Some(Subs::Del(arg)) => {
            dbg!(arg);
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
            dbg!(arg);
        },
        Some(Subs::Init) => todo!(),
        None => print_todo(path),
    }
}
