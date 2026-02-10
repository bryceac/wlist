use clap::Parser;
use wlitem::Item;
use crate::note::Note;

use crate::{database::{create_database_if_not_exists, load_items_from_db, load_notes_from_db, item_with_id}, content::Content};

#[derive(Parser)]
#[clap(version = "0.1.0", author = "Bryce Campbell <tonyhawk2100@gmail.com>", about = "display wishlist content.", long_about = "Display the contents of the wishlist, whether that be:\r\n\r\n* the notes\r\n* the items\r\n* notes attached to items.\r\n\r\nThe default is the list of items.\r\n\r\nTo view the notes, you would do something like this:\r\n\r\nwlist show path/to/database notes\r\n\r\nThis will display all the notes in the wish list, and if you provide an item id, it will show you the notes belonging to that item.")]
pub struct Show {
    #[clap(default_value = "~/wishlist/gift_registry.db", help = "the path to the wishlist database")]
    pub file_path: String,

    #[clap(value_enum, help = "the content to display", default_value_t=Content::Items)]
    pub content: Content,

    #[clap(long, short, help = "the id of the item to look at.")]
    pub item_id: Option<String>
}

impl Show {
    pub fn run(&self) {
        create_database_if_not_exists(&self.file_path);

        match self.content {
            Content::Items => {
                if self.item_id.is_some() {
                    println!("Item id is not allowed to be specified when displaying items.");
                    return;
                }

                let mut item_store = load_items_from_db(&self.file_path);
        
                item_store.sort_by_key(|item| item.priority.clone());

                item_store.reverse();

                display_items(&item_store);
            },
            Content::Notes => {
                if let Some(id) = self.item_id.clone() {
                    let notes = if let Some(item) = item_with_id(&self.file_path, &id) {
                        item.notes
                    } else {
                        vec![]
                    };

                    for note in notes {
                        println!("{}\r\n\r\n", note);
                    }
                } else {
                    let notes = load_notes_from_db(&self.file_path);

                    display_notes(&notes);
                }
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

fn display_notes(store: &Vec<Note>) {
    for note in store {
        println!("{}\r\n-----\r\n\r\n{}\r\n\r\n", note.id, note.note);
    }
}