use clap::Parser;
use crate::add::Add;

#[derive(Parser)]
pub enum SubCommand {
    Add(Add)
}