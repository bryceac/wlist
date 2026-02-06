mod shared;
mod database;
mod note;
mod wlist;
mod add;

use wlist::WList;
use clap::Parser;

fn main() {
    let wishlist = WList::parse();
}
