use std::{ fs::File, io::{ self, Write }, path::Path };
use clap::Parser;
use wlitem::{ Item, Save };
use indexmap::IndexSet;
use icu::locale::locale;
use xlsxwriter::prelude::*;

use crate::{shared::real_path, database::{load_items_from_db, create_database_if_not_exists} };

#[derive(Parser)]
#[clap(version = "0.1.3", author = "Bryce Campbell <tonyhawk2100@gmail.com>", about = "export wishlist.", long_about = "export wishlist to file. Supported formats are as follows:\r\n\r\n* JSON (exports items and notes)\r\n* TSV (exports only items and basic details)\r\n* HTML (file that can be distributed to others with stored information)\r\n\r\nPlease be aware that the HTML output is very basic and is only meant to be a\r\nstarting point, if you find that it does not look that great.\r\n\r\nThe data exported is determined by the extension and exports to TSV by default.")]
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
            ref p if p.ends_with(".ods") => if let Err(error) = spreadsheet_ods::write_ods(&mut create_ods_book(items.clone()), Path::new(&destination_path)) {
                match error {
                    spreadsheet_ods::OdsError::Io(error) => println!("{}", error),
                    spreadsheet_ods::OdsError::Utf8(error) => println!("{}", error),
                    spreadsheet_ods::OdsError::Xml(error) => println!("{}", error),
                    spreadsheet_ods::OdsError::Zip(error) => println!("{}", error),
                    _ => ()
                }
            }
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
    html.push_str(&format!("\t\t<title>{}</title>\r\n", title));
    html.push_str("\t</head>\r\n");
    html.push_str("\t<body>\r\n");
    html.push_str("\t\t<article>\r\n");
    html.push_str("\t\t\t<header>\r\n");
    html.push_str(&format!("\t\t\t\t<h1>{}</h1>\r\n", title));
    html.push_str("\t\t\t\t<hr>\r\n");
    html.push_str("\t\t\t</header>\r\n");
    html.push_str(&registry(items));
    if !unique_notes(&items).is_empty() {
        html.push_str("\t\t\t<footer>\r\n");
        html.push_str(&format!("\t\t\t\t<h2>Notes</h2>\r\n"));
        html.push_str("\t\t\t\t<hr>\r\n");
        html.push_str(&note_list(items));
        html.push_str("\t\t\t</footer>\r\n");
    }
    html.push_str("\t\t</article>\r\n");
    html.push_str("\t</body>\r\n");
    html.push_str("</html>");

    html
}

fn unique_notes(items: &Vec<Item>) -> IndexSet<String> {
    let mut notes = IndexSet::new();

    for item in items.clone() {
        for note in item.notes.clone() {
            notes.insert(note);
        }
    }

    notes
}

fn note_list(items: &Vec<Item>) -> String {
    let notes = unique_notes(items);

    let mut note_string = "\t\t\t\t<ol id=\"notes\">\r\n".to_owned();

    for (position, note) in notes.iter().enumerate() {
        let id_number = position+1;

        note_string.push_str(&format!("\t\t\t\t\t<li id=\"note{}\">{}</li>\r\n", id_number, note));

    }

    note_string.push_str("\t\t\t\t</ol>\r\n");

    note_string
}

fn registry(items: &Vec<Item>) -> String {
    let mut item_string = "\t\t\t<ol id=\"registry\">\r\n".to_owned();

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

fn create_ods_book(items: Vec<Item>) -> spreadsheet_ods::WorkBook {
    let mut book = spreadsheet_ods::WorkBook::new(locale!("en_US"));

    let mut registry_sheet = spreadsheet_ods::Sheet::new("Wishlist");
    let mut note_sheet = spreadsheet_ods::Sheet::new("Notes");
    let mut link_sheet = spreadsheet_ods::Sheet::new("Links");
    let notes = unique_notes(&items);

    registry_sheet.set_value(0, 1, "Name");
    registry_sheet.set_value(0, 2, "Quantity");
    registry_sheet.set_value(0, 3, "Priority");
    registry_sheet.set_value(0, 4, "URL");

    link_sheet.set_value(0, 0, "Item ID");
    link_sheet.set_value(0, 1, "Note ID");

    for (index, item) in items.iter().enumerate() {
        let row_index = index+1;

        add_item_to_ods_sheet(item, row_index.try_into().unwrap(), &mut registry_sheet);
    }

    book.push_sheet(registry_sheet);

    for (index, note) in notes.iter().enumerate() {
        note_sheet.set_value(index.try_into().unwrap(), 0, note.clone());
    }

    book.push_sheet(note_sheet);

    ods_items_to_notes_link(&items, &notes, &mut link_sheet);

    book.push_sheet(link_sheet);

    book
}

fn add_item_to_ods_sheet(item: &Item, row_index: u32, sheet: &mut spreadsheet_ods::Sheet) {
    sheet.set_value(row_index, 0, item.id.clone());
    sheet.set_value(row_index, 1, item.name.clone());
    sheet.set_value(row_index, 2, item.quantity);
    sheet.set_value(row_index, 3, item.priority.to_str());

    if let Some(url) = item.url.clone() {
        sheet.set_value(row_index, 4, url.to_string());
    }
}

fn ods_items_to_notes_link(items: &Vec<Item>, notes: &IndexSet<String>, sheet: &mut spreadsheet_ods::Sheet) {
    let mut note_offset: Option<u32> = None;

    for (index, item) in items.iter().enumerate() {
        let start_index = if let Some(note_offset) = note_offset {
            (index as u32) + note_offset
        } else {
            (index+1) as u32
        };

        for (note_index, note) in notes.iter().enumerate() {
            let row_index = start_index + (note_index as u32);

            if item.notes.contains(note) {
                sheet.set_value(row_index, 0, item.id.clone());
                sheet.set_value(row_index, 1, (note_index+1) as i32);
            }
        }

        note_offset = Some(item.notes.len().try_into().unwrap());
    }
}

fn create_xlsx_book(p: &str, items: Vec<Item>) -> Result<(), XlsxError> {
    let mut book = Workbook::new(p)?;

    let mut registry_sheet = book.add_worksheet(Some("Wishlist"))?;
    let mut note_sheet = book.add_worksheet(Some("Notes"))?;
    let mut link_sheet = book.add_worksheet(Some("Links"))?;

    registry_sheet.write_blank(0, 0, None)?;
    registry_sheet.write_string(0, 1, "Name", None)?;
    registry_sheet.write_string(0, 2, "Quantity", None)?;
    registry_sheet.write_string(0, 3, "Priority", None)?;
    registry_sheet.write_string(0, 4, "URL", None)?;

    link_sheet.write_string(0, 0, "Item ID", None)?;
    link_sheet.write_string(0, 1, "Note ID", None)?;

    for (index, item) in items.iter().enumerate() {
        let row_index = index+1;

        let _ = add_item_to_xlsx_sheet(&item, row_index.try_into().unwrap(), &mut registry_sheet);
    }

    book.close()?;
    Ok(())
}

fn add_item_to_xlsx_sheet(item: &Item, row_index: u32, sheet: &mut Worksheet) -> Result<(), XlsxError> {
    sheet.write_string(row_index, 0, &item.id.clone(), None)?;
    sheet.write_string(row_index, 1, &item.name.clone(), None)?;
    sheet.write_string(row_index, 2, &item.quantity.to_string(), None)?;
    sheet.write_string(row_index, 3, item.priority.to_str(), None)?;

    if let Some(url) = item.url.clone() {
        sheet.write_string(row_index, 4, &url.to_string(), None)
    } else {
        sheet.write_blank(row_index, 4, None)
    }
}