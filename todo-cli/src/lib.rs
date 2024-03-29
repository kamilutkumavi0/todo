pub mod clap_structure;
use clap::Parser;
use clap_structure::Todo;
pub fn parse_it()->Todo{
    Todo::parse()
}
