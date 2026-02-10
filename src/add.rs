use clap::Parser;
use wlitem::{Priority, Item};

use crate::database::{create_database_if_not_exists, add_item};

#[derive(Parser)]
#[clap(version = "0.1.0", author = "Bryce Campbell <tonyhawk2100@gmail.com>", about = "add new item to wishlist.", long_about = " add new item to wishlist. Items are added by simply passing the necessary data.\r\n\r\nOf that data, only the a name is required by doing something like this:\r\n\r\nwlist -n Computer\r\n\r\nIf there are spaces in the name, the name should be enclosed in quotes.\r\n\r\nOther parameters can be specified include the following:\r\n\r\n* Quantity\r\n* Priority (specifies the degree to which you the item)\r\n* URL (place to buy the item)\r\n* Notes\r\n\r\nAll of these are optional and have default values specified if none are provided.\r\n\r\nLike the name, if notes contain spaces, they should be enclosed in quotes. Otherwise, they will be considered separate notes.")]
pub struct Add {
    #[clap(default_value = "~/wishlist/gift_registry.db", help = "the path to the wishlist database")]
    pub file_path: String,

    #[clap(long, short)]
    pub name: String,

    #[clap(long, short, default_value = "1")]
    pub quantity: u32,

    #[clap(long, short, default_value = "low")]
    pub priority: Priority,

    #[clap(long, short)]
    pub url: Option<String>,

    #[clap(long, num_args = 0..)]
    pub notes: Option<Vec<String>>
}

impl Add {
    pub fn run(&self) {
        create_database_if_not_exists(&self.file_path);

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