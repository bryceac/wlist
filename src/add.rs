use clap::Parser;
use wlitem::{Priority, Item};

use crate::database::{copy_database_if_not_exists, add_item};

#[derive(Clone, Parser)]
#[clap(version = "0.1.0", author = "Bryce Campbell <tonyhawk2100@gmail.com>")]
pub struct Add {
    #[clap(default_value = "~/wishlist/gift_registry.db")]
    pub file_path: String,

    #[clap(long, short)]
    pub name: String,

    #[clap(long, short, default_value = "1")]
    pub quantity: u32,

    #[clap(long, short, default_value = "medium")]
    pub priority: Priority,

    #[clap(long, short, default_value = "")]
    pub url: String,

    #[clap(long, num_args = 0.., value_delimiter = ',', required = false)]
    pub notes: Vec<String>
}

impl Add {
    pub fn run(&self) {
        copy_database_if_not_exists(&self.file_path);

        self.add_item(&self.file_path);
    }

    fn add_item(&self, p: &str) {
        let item = Item::from("", 
        &self.name, 
        self.quantity, 
        self.priority.to_str(), 
        &self.url, 
        self.notes.clone());

        add_item(p, item);
    }
}