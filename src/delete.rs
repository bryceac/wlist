use clap::Parser;

#[derive(Parser)]
#[clap(version = "0.1.0", author = "Bryce Campbell <tonyhawk2100@gmail.com>", long_about = "delete items or notes.")]
pub struct Delete {
    #[clap(default_value = "~/wishlist/gift_registry.db")]
    pub file_path: String,

    #[clap(long, short)]
    pub item_id: Option<String>,

    #[clap(long, short)]
    pub note_id: Option<u32>
}

impl Delete {
    pub fn run(&self) {

    }
}