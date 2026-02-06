use clap::Parser;
use wlitem::Item;

use crate::{database::{copy_database_if_not_exists, load_items_from_db, load_notes_from_db}, content::Content};

#[derive(Parser)]
#[clap(version = "0.1.0", author = "Bryce Campbell <tonyhawk2100@gmail.com>", long_about = "display wishlist content.")]
pub struct Show {
    #[clap(default_value = "~/wishlist/gift_registry.db")]
    pub file_path: String,

    #[clap(value_enum, default_value_t=Content::Items)]
    pub content: Content

    #[clap(long, short)]
    pub item_id: Option<String>
}

impl Show {
    pub fn run(&self) {
        copy_database_if_not_exists(&self.file_path);

        match self.content {
            Content::Items => {
                if self.item_id.is_some() {
                    println!("Item id is not allowed to be specified when displaying items.");
                    return;
                }

                let item_store = load_items_from_db(&self.file_path);

                display_items(&item_store);
            },
            Content::Notes => {
                let notes = load_notes_from_db(&self.file_path);
            }
        }
    }
}

fn display_items(store: &Vec<Item>) {
    for item in store {
        let item_url = if let Some(url) = item.url.clone() {
            url.as_str().to_owned()
        } else {
            "N/A".to_owned()
        };

        println!("{}\t{},\t{}\t{}\t{}", 
        item.id, 
        item.name, 
        item.quantity, 
        item.priority.to_str(), 
        item_url);
    }
}