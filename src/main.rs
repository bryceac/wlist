mod shared;
mod database;
mod note;
mod wlist;

use wlist::WList;
use clap::Parser;

fn main() {
    let wishlist = WList::parse();
}
