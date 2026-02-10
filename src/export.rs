use std::{ collections::HashSet, fs::File, io::{ self, Write } };
use clap::Parser;
use wlitem::{ Item, Save };

use crate::{shared::real_path, database::{load_items_from_db, create_database_if_not_exists} };

#[derive(Parser)]
#[clap(version = "0.1.0", author = "Bryce Campbell <tonyhawk2100@gmail.com>", about = "export wishlist.", long_about = "export wishlist to file. Supported formats are as follows:\r\n\r\n* JSON (exports items and notes)\r\n* TSV (exports only items and basic details)\r\n* HTML (file that can be distributed to others with stored information)\r\n\r\nPlease be aware that the HTML output is very basic and is only meant to be a\r\nstarting point, if you find that it does not look that great.\r\n\r\nThe data exported is determined by the extension and exports to TSV by default.")]
pub struct Export {
    #[clap(default_value = "~/wishlist/gift_registry.db", help = "the path to the wishlist database")]
    pub file_path: String,

    #[clap(long, short, help = "set title of HTML output", default_value = "Wishlist")]
    pub title: String,

    #[clap(long, short, help = "the file that the content is supposed to be saved to.")]
    pub output_file: String
}

impl Export {
    pub fn run(&self) {
        create_database_if_not_exists(&self.file_path);
        let destination_path = real_path(&self.output_file);
        let mut items: Vec<Item> = load_items_from_db(&self.file_path);
        
        items.sort_by_key(|item| item.priority.clone());

        items.reverse();

        match destination_path {
            ref p if p.ends_with(".json") => if let Err(error) = items.save(p) {
                println!("{}", error);
            },
            ref p if p.ends_with(".html") => if let Err(error) = save_html(items, &self.title, &real_path(p)) {
                println!("{}", error);
            },
            _ => if let Err(error) = items.save_tsv(&destination_path) {
                println!("{}", error);
            }
        }
    }
}

fn generate_html(items: &Vec<Item>, title: &str) -> String {
    let mut html = "<!DOCTYPE html>\r\n".to_owned();

    html.push_str("<html>\r\n");

    html.push_str("\t<head>\r\n");
    html.push_str(&format!("\t\t<title>{}</title>", title));
    html.push_str("\t</head>\r\n");
    html.push_str("\t<body>\r\n");
    html.push_str("\t\t<article>");
    html.push_str("\t\t\t<header>");
    html.push_str(&format!("\t\t\t\t<h1>{}</h1>", title));
    html.push_str("\t\t\t</header>");
    html.push_str("\t\t\t<hr>");
    html.push_str(&registry(items));
    html.push_str("\t\t\t<footer>");
    html.push_str(&format!("\t\t\t\t<h2>Notes</h2>"));
    html.push_str("\t\t\t\t<hr>");
    html.push_str(&note_list(items));
    html.push_str("\t\t\t</footer>");
    html.push_str("\t\t</article>");
    html.push_str("\t</body>\r\n");
    html.push_str("</html>");

    html
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

fn note_list(items: &Vec<Item>) -> String {
    let notes = unique_notes(items);

    let mut note_string = "\t\t\t\t<ol>\r\n".to_owned();

    for (position, note) in notes.iter().enumerate() {
        let id_number = position+1;

        note_string.push_str(&format!("\t\t\t\t\t<li id=\"note{}\">{}</li>\r\n", id_number, note));

    }

    note_string.push_str("\t\t\t\t</ol>\r\n");

    note_string
}

fn registry(items: &Vec<Item>) -> String {
    let mut item_string = "\t\t\t<ol>\r\n".to_owned();

    for item in items {
        item_string.push_str(&format!("\t\t\t\t<li>{}</li>\r\n", &registry_item(item, items)));
    }

    item_string.push_str("\t\t\t</ol>\r\n");

    item_string
}

fn registry_item(item: &Item, items: &Vec<Item>) -> String {
    let mut details = if item.quantity > 1 {
        format!("{} ", item.quantity)
    } else {
        "".to_owned()
    };

    let notes = unique_notes(items);

    if let Some(url) = item.url.clone() {
        if url.to_string().is_empty() {
            details.push_str(&item.name);
        } else {
            details.push_str(&format!("<a href=\"{}\">{}</a>", url.to_string(), item.name));
        }
    } else {
        details.push_str(&item.name);
    }

    if !item.notes.is_empty() {
        details.push_str(" ");

        for (position, note) in notes.iter().enumerate() {
            if item.notes.contains(note) {
                let id_number = position+1;
                let destination = format!("#note{}", id_number);
                details.push_str(&format!("<sup>[<a href=\"{}\">{}</a>]</sup>", destination, id_number));
            }
        }
    }

    details
}

fn save_html(items: Vec<Item>, title: &str, p: &str) -> Result<(), io::Error> {
    let html = generate_html(&items, title);

    let mut output = File::create(p)?;

    match write!(output, "{}", html) {
        Ok(()) => Ok(()),
        Err(error) => Err(error)
    }
}