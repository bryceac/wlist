mod shared;
mod database;
mod note;
mod wlist;
mod add;
mod subcommand;

use wlist::WList;
use clap::Parser;

fn main() {
    let wishlist = WList::parse();
}
