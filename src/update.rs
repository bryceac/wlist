use clap::Parser;
use wlitem::Priority;
use url::Url;

use crate::database::*;

#[derive(Parser)]
#[clap(version = "0.1.0", author = "Bryce Campbell <tonyhawk2100@gmail.com>", about = "update items and notes.", long_about = "update items or notes.\r\n\r\nBy supplying the right data, you can do either of the following:\r\n\r\n* update notes\r\n* update item details\r\n* append notes to items\r\n* remove notes from items\r\n\r\nPlease note that the latter two options cannot be performed in conjunction with the former two.\r\n\r\nModifying items is a lot like adding them, especially because notes can only be added through items, so things work in the same capacity here.\r\n\r\nNote modification can be done by supplying the identifier for the note, which can be found by running the show command.\r\nThis is the only way notes can be updated.\r\n\r\nIf you want to add or remove existing notes from item, provide the ids for both the iitem and the note like this:\r\n\r\nwlist update -i 15278603-03F1-41E0-81ED-6E94883F9AC7 -n 1\r\n\r\nAfter that, you need to pass either the -r flag to break the connection between them or the -a flag to link them together.\r\n\r\nItem identifiers, like the identifiers for notes can be be found with the show command.")]
pub struct Update {
    #[clap(default_value = "~/wishlist/gift_registry.db", help = "the path to the wishlist database")]
    pub file_path: String,

    #[clap(long, short, help = "item identifier")]
    pub item_id: Option<String>,

    #[clap(long, short, help = "note identifier")]
    pub note_id: Option<u32>,

    #[clap(long)]
    pub name: Option<String>,

    #[clap(long, short)]
    pub quantity: Option<u32>,

    #[clap(long, short)]
    pub priority: Option<Priority>,

    #[clap(long, short)]
    pub url: Option<String>,

    #[clap(long, num_args = 0..)]
    pub notes: Option<Vec<String>>,

    #[clap(long)]
    pub note: Option<String>,

    #[clap(long, short)]
    pub remove_note: bool,

    #[clap(long, short)]
    pub append_note: bool
}

impl Update {
    pub fn run(&self) {
        create_database_if_not_exists(&self.file_path);

        if self.item_id.is_some() && 
        self.note_id.is_some() && 
        self.note.is_some() {
            println!("note cannot be specified if both note id and item id are given. If you intend to update a note, please only specify the note id");
            return;
        }

        if self.item_id.is_some() &&
        self.note_id.is_some() {
            if self.name.is_some() &&
            self.quantity.is_some() &&
            self.priority.is_some() &&
            self.url.is_some() &&
            self.notes.is_some() {
                println!("Cannot update item details and append\r\n or remove existng notes. Please only specify if you want to remove or add note.\r\n\r\nOtherwise, only specify an item or note id.");
                return;
            }

            if let Some(id) = self.item_id.clone() {
                if let Some(item) = item_with_id(&self.file_path, &id) {
                    if let Some(note_id) = self.note_id {
                        if !self.remove_note && !self.append_note || self.remove_note && self.append_note {
                            println!("Purpose of having a note id is not clear.\r\nPlease use ONE flag to determine if the note is to be appended or removed.");
                            return;
                        }
        
                        if self.remove_note {
                            remove_note_from_item(&self.file_path, &item, note_id);
                        }
        
                        if self.append_note {
                            if let Some(note) = note_with_id(&self.file_path, note_id) {
                                link_note_to_item(&self.file_path, &item, &note.note);
                            }
                        }
                    }
                }
            }
        }

        if let Some(item_id) = self.item_id.clone() {
            if let Some(mut item) = item_with_id(&self.file_path, &item_id) {
                if let Some(name) = self.name.clone() {
                    item.name = name;
                }

                if let Some(quantity) = self.quantity {
                    item.quantity = quantity;
                }

                if let Some(priority) = self.priority.clone() {
                    item.priority = priority;
                }

                if let Some(url_string) = self.url.clone() {
                    if let Ok(url) = Url::parse(&url_string) {
                        item.url = Some(url);
                    }
                }

                if let Some(notes) = self.notes.clone() {
                    for note in notes.clone() {

                        if !notes.clone().contains(&note) {
                            item.notes.push(note);
                        }
                    }
                }

                update_item(&self.file_path, &item);
            }
        }

        if let Some(note_id) = self.note_id {
            if let Some(note) = self.note.clone() {
                update_note_with_id(&self.file_path, note_id, &note);
            }
        }
    }
}