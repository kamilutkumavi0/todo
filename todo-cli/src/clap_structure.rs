use clap::{Parser, Args, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Todo{
    #[command(subcommand)]
    pub sub_command: Option<Subs>,
    #[arg(default_value_t=String::from("./todo.td"))]    
    pub path: String,
}
#[derive(Subcommand, Debug)]
pub enum Subs{
    Add(AddArgs),
    Update(UpdateArgs),
    Del(DelArgs),
    Init,
}

#[derive(Args, Debug)]
pub struct UpdateArgs{
    pub name: String,
    #[arg(long, short = 'S', value_enum,default_value_t=Status::NotStarted)]
    pub status: Status,
}

#[derive(Args, Debug)]
pub struct DelArgs{
    pub name: String,
}



#[derive(Args, Debug)]
pub struct AddArgs{
    pub name: String,
    
    #[arg(long, short)]
    pub description: Option<String>,

    #[arg(long, short)]
    pub start_date: Option<String>,

    #[arg(long, short)]
    pub finish_date: Option<String>,
    
    #[arg(long, short = 'S', value_enum,default_value_t=Status::NotStarted)]
    pub status: Status,
}

#[derive(ValueEnum, Debug, Clone)]
pub enum Status{
    NotStarted,
    Continue,
    Done,
}
