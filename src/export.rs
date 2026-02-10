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
        todo!();
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

fn notes_for_item(item: &Item, items: &Vec<Item>) -> Vec<HtmlElement> {
    let mut note_elements: Vec<HtmlElement> = vec![];
    let notes = unique_notes(items);

    for (position, note) in notes.iter().enumerate() {
        if item.notes.contains(note) {
            let id_number = position+1;
            let link_destination = format!("#note{}", id_number);
            let note_element = HtmlElement::new(HtmlTag::Superscript)
            .with_link(link_destination, id_number.to_string().as_str());

            note_elements.push(note_element);
        }
    }

    note_elements
}

fn list_element(item: &Item, items: &Vec<Item>) -> HtmlElement {
    let mut list_item = HtmlElement::new(HtmlTag::ListElement);
    let item_note_elements = notes_for_item(item, items);

    /* let mut item_without_url_details = if item.quantity > 1 {
        format!("{} {}", item.quantity, item.name)
    } else {
        format!("{}", item.name)
    };

    if !item_note_elements.is_empty() {
        item_without_details.push_str(" ");

        for note_element in item_note_elements {
            unlinked_item_details.push_str(&note_element.to_html_string());
        }
    } */

    if item.quantity > 1 {
        list_item
        .with_child(item.quantity.to_string().as_str().into())
        .with_child(" ".into());
    }

    

    if let Some(url) = item.url {
        if !url.to_string().is_empty() {
            list_item
            .with_link(href, text)
        }
    }

    list_item
}