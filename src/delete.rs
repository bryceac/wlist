use clap::Parser;

use crate::database::{delete_item, item_with_id, delete_note_with_id, create_database_if_not_exists};

#[derive(Parser)]
#[clap(version = "0.1.0", author = "Bryce Campbell <tonyhawk2100@gmail.com>", about = "delete items or notes.", long_about = "Delete items or notes from the wishlist.\r\n\r\nNotes and items can be deleted by passing the respective id, which can be \r\ndiscoered through the show command.\r\n\r\nHowever, you can only delete one or the other at a time.")]
pub struct Delete {
    #[clap(default_value = "~/wishlist/gift_registry.db", help = "the path to the wishlist database")]
    pub file_path: String,

    #[clap(long, short)]
    pub item_id: Option<String>,

    #[clap(long, short)]
    pub note_id: Option<u32>
}

impl Delete {
    pub fn run(&self) {
        create_database_if_not_exists(&self.file_path);
        if self.item_id.is_some() && self.note_id.is_some() {
            println!("You can only either delete notes or items. Please specify only one id.");
            return;
        }

        if let Some(item_id) = self.item_id.clone() {
            if let Some(item) = item_with_id(&self.file_path, &item_id) {
                delete_item(&self.file_path, &item);
            }
        }

        if let Some(note_id) = self.note_id {
            delete_note_with_id(&self.file_path, note_id);
        }
    }
}