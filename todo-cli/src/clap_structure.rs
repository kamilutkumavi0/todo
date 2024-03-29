use clap::{Parser, Args, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Todo{
    #[command(subcommand)]
    sub_command: Option<Subs>,
    path: String,
}
#[derive(Subcommand, Debug)]
pub enum Subs{
    Create(CreateArgs),
    Update(UpdateArgs),
    Delete(DeleteArgs),
}

#[derive(Args, Debug)]
struct UpdateArgs{
    name: String,
    #[arg(long, short = 'S', value_enum,default_value_t=Status::NotStarted)]
    status: Status,
}

#[derive(Args, Debug)]
struct DeleteArgs{
    name: String,
}



#[derive(Args, Debug)]
struct CreateArgs{
    name: String,
    
    #[arg(long, short)]
    description: Option<String>,

    #[arg(long, short)]
    start_date: Option<String>,

    #[arg(long, short)]
    finish_date: Option<String>,
    
    #[arg(long, short = 'S', value_enum,default_value_t=Status::NotStarted)]
    status: Status,
}

#[derive(ValueEnum, Debug, Clone)]
enum Status{
    NotStarted,
    Continue,
    Done,
}
