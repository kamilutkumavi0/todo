use todo_cli::{parse_it, clap_structure::{Status, AddArgs, DelArgs, UpdateArgs, Subs, Todo}};

pub fn user_input(){
    let parsed_arg = parse_it();
	let path = parsed_arg.path;
    match parsed_arg.subs{
        Subs::Del(arg) => {
            dbg!(arg);
        },
        Subs::Add(arg) => {
            dbg!(arg);
        },
        Subs::Update(arg) => {
            dbg!(arg);
        },
    }
}
