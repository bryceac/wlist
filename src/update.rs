use clap::Parser;
use wlitem::Priority;
use url::Url;

use crate::database::*;

#[derive(Parser)]
#[clap(version = "0.1.0", author = "Bryce Campbell <tonyhawk2100@gmail.com>", about = "update items and notes.")]
pub struct Update {
    #[clap(default_value = "~/wishlist/gift_registry.db")]
    pub file_path: String,

    #[clap(long, short)]
    pub item_id: Option<String>,

    #[clap(long, short)]
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
        copy_database_if_not_exists(&self.file_path);

        if self.item_id.is_some() && 
        self.note_id.is_some() && 
        self.note.is_some() {
            println!("note cannot be specified if both note id and item id are given. If you intend to update a note, please only specify the note id");
            return;
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

                if let Some(note_id) = self.note_id {
                    if !self.remove_note && !self.append_note {
                        println!("Purpose of having a note id is not clear.\r\nPlease use a flag to determine if the note is to be appended or removed.");
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