use clap::Parser;
use wlitem::Item;

use crate::shared::real_path;
use crate::database::{ copy_database_if_not_exists, update_or_add_item };

#[derive(Parser)]
#[clap(version = "0.1.0", author = "Bryce Campbell <tonyhawk2100@gmail.com>", about = "import wishlist.")]
pub struct Import {
    #[clap(default_value = "~/wishlist/gift_registry.db")]
    pub file_path: String,

    #[clap(long, short)]
    pub input_file: String
}

impl Import {
    pub fn run(&self) {
        copy_database_if_not_exists(&self.file_path);
        let origin_path = real_path(&self.input_file);
        
        let items = match origin_path {
            ref p if p.ends_with(".json") => if let Ok(decoded_items) = Item::from_file(p) {
                decoded_items
            } else {
                vec![]
            },
            _ => if let Ok(decoded_items) = Item::from_tsv_file(&self.file_path) {
                decoded_items
            } else {
                vec![]
            }
        };

        for item in items {
            update_or_add_item(&self.file_path, item);
        }
    }
}
