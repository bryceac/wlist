mod shared;
mod database;
mod note;
mod wlist;
mod add;
mod subcommand;
mod show;

use subcommand::SubCommand;
use wlist::WList;
use clap::Parser;

fn main() {
    let wishlist = WList::parse();

    match wishlist.subcommand {
        SubCommand::Add(a) => a.run(),
        SubCommand::Show(s) => s.run()
    }
}
