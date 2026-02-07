use clap::Parser;
use crate::add::Add;
use crate::delete::Delete;
use crate::show::Show;
use crate::export::Export;
use crate::import::Import;
use crate::update::Update;

#[derive(Parser)]
pub enum SubCommand {
    Add(Add),
    Show(Show),
    Export(Export),
    Import(Import),
    Delete(Delete),
    Update(Update)
}