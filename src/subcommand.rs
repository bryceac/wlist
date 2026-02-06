use clap::Parser;
use crate::add::Add;
use crate::show::Show;
use crate::export::Export;

#[derive(Parser)]
pub enum SubCommand {
    Add(Add),
    Show(Show),
    Export(Export)
}