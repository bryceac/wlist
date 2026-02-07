use clap::Parser;
use wlitem::Priority;

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