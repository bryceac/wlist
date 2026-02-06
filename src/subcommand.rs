use clap::Parser;
use crate::add::Add;
use crate::show::Show;

#[derive(Parser)]
pub enum SubCommand {
    Add(Add),
    Show(Show)
}