use clap::Parser;
use wlitem::{Priority, Item};

use crate::database::{copy_database_if_not_exists, add_item};

#[derive(Parser)]
#[clap(version = "0.1.0", author = "Bryce Campbell <tonyhawk2100@gmail.com>", long_about = "add new item to wishlist.")]
pub struct Add {
    #[clap(default_value = "~/wishlist/gift_registry.db")]
    pub file_path: String,

    #[clap(long, short)]
    pub name: String,

    #[clap(long, short, default_value = "1")]
    pub quantity: u32,

    #[clap(long, short, default_value = "medium")]
    pub priority: Priority,

    #[clap(long, short)]
    pub url: Option<String>,

    #[clap(long, num_args = 0..)]
    pub notes: Option<Vec<String>>
}

impl Add {
    pub fn run(&self) {
        copy_database_if_not_exists(&self.file_path);

        self.add_item_to_db(&self.file_path);
    }

    fn add_item_to_db(&self, p: &str) {
        let mut item_builder = Item::builder();

        item_builder.set_name(&self.name)
        .set_quantity(self.quantity)
        .set_priority(self.priority.to_str());

        if let Some(url) = self.url.clone() {
            item_builder.set_url(&url);
        }

        if let Some(notes) = self.notes.clone() {
            for note in notes {
                item_builder.add_note(&note);
            }
        }

        let item = item_builder.build();

        add_item(p, item);
    }
}