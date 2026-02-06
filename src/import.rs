use clap::Parser;
use wlitem::Item;

use crate::shared::real_path;

#[derive(Parser)]
#[clap(version = "0.1.0", author = "Bryce Campbell <tonyhawk2100@gmail.com>", long_about = "import wishlist.")]
pub struct Import {
    #[clap(default_value = "~/wishlist/gift_registry.db")]
    pub file_path: String,

    #[clap(long, short)]
    pub input_file: String
}

impl Import {
    pub fn run(&self) {
        let origin_path = real_path(&self.input_file);
        let items = match origin_path {
            ref p if p.ends_with(".json") => if let Err(error) = Item::from_file(p) {
                println!("{}", error);
            },
            _ => if let Err(error) = Item::from_tsv_file(p) {
                println!("{}", error)
            }
        }
    }
}