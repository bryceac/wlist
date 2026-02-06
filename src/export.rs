use clap::Parser;
use wlitem::{ Item, Save };

use crate::{shared::real_path, database::load_items_from_db};

#[derive(Parser)]
#[clap(version = "0.1.0", author = "Bryce Campbell <tonyhawk2100@gmail.com>", long_about = "export wishlist.")]
pub struct Export {
    #[clap(default_value = "~/wishlist/gift_registry.db")]
    pub file_path: String,

    #[clap(long, short)]
    pub output_file: String
}

impl Export {
    pub fn run(&self) {
        let destination_path = real_path(&self.output_file);
        let items = load_items_from_db(&self.file_path);

        match destination_path {
            p if p.ends_with(".json") => {},
            _ => {}
        }
    }
}