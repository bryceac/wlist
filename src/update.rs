use clap::Parser;
use wlitem::Priority;
use crate::note::Note;

use crate::database::copy_database_if_not_exists;

#[derive(Parser)]
#[clap(version = "0.1.0", author = "Bryce Campbell <tonyhawk2100@gmail.com>", long_about = "update items and notes.")]
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

    #[clap(long)]
    pub notes: Option<Vec<Note>>,

    #[clap(long)]
    pub note: Option<String>
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
    }
}