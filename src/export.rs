use clap::Parser;
use wlitem::Save;

use crate::{shared::real_path, database::{load_items_from_db, create_database_if_not_exists}};

#[derive(Parser)]
#[clap(version = "0.1.0", author = "Bryce Campbell <tonyhawk2100@gmail.com>", about = "export wishlist.")]
pub struct Export {
    #[clap(default_value = "~/wishlist/gift_registry.db")]
    pub file_path: String,

    #[clap(long, short, help = "set title of HTML output", default_value = "Wishlist")]
    pub title: String,

    #[clap(long, short)]
    pub output_file: String
}

impl Export {
    pub fn run(&self) {
        create_database_if_not_exists(&self.file_path);
        let destination_path = real_path(&self.output_file);
        let items = load_items_from_db(&self.file_path);

        match destination_path {
            ref p if p.ends_with(".json") => if let Err(error) = items.save(p) {
                println!("{}", error);
            },
            _ => if let Err(error) = items.save_tsv(&destination_path) {
                println!("{}", error)
            }
        }
    }
}

fn generate_html() -> String {
    let html_string = build_html::HtmlPage::new()

}