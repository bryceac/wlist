use clap::Parser;
use wlitem::Item;

use crate::shared::real_path;
use crate::database::{ create_database_if_not_exists, update_or_add_item };

#[derive(Parser)]
#[clap(version = "0.1.0", author = "Bryce Campbell <tonyhawk2100@gmail.com>", about = "import wishlist.", long_about = "import items to the wishlist.\r\n\r\nUnlike the export command, only two formats are supported, which are as follows:\r\n\r\n* JSON (imports item details and notes)\r\n*TSV (only imports item details).\r\n\r\nIf the imported data contains items that already exist, they will be updated.\r\n\r\nHowever, notes in a JSON file will not be updated and will be added as new notes.\r\n\r\nImports are done based on file extension, with TSV being the default.")]
pub struct Import {
    #[clap(default_value = "~/wishlist/gift_registry.db", help = "the path to the wishlist database")]
    pub file_path: String,

    #[clap(long, short, help = "the file to import data from.")]
    pub input_file: String
}

impl Import {
    pub fn run(&self) {
        create_database_if_not_exists(&self.file_path);
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
            update_or_add_item(&self.file_path, &item);
        }
    }
}
