use std::collections::HashSet;

use build_html::{ HtmlContainer, HtmlPage, HtmlTag, HtmlChild, HtmlElement, Html};
use clap::Parser;
use wlitem::{ Item, Save };

use crate::{shared::real_path, database::{load_items_from_db, create_database_if_not_exists} };

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

fn generate_html(items: Vec<Item>, title: &str) -> String {
    let mut html_page = HtmlPage::new()
    .with_title(title)
    .with_header(1, title)
    .with_html(HtmlElement::new(HtmlTag::HorizontalRule));

    let mut item_list = HtmlElement::new(HtmlTag::OrderedList);

    for item in items {
        let mut item_details = format!("{}", item.quantity);


        let mut list_item = HtmlElement::new(HtmlTag::ListElement);
        item_list.with_child(HtmlChild::Element(list_item));
    }

    html_page
    .with_header(2, "Notes")
    .with_html(HtmlElement::new(HtmlTag::HorizontalRule));

    html_page.to_html_string()
}

fn unique_notes(items: &Vec<Item>) -> HashSet<String> {
    let mut notes = HashSet::new();

    for item in items.clone() {
        for note in item.notes.clone() {
            notes.insert(note);
        }
    }

    notes
}