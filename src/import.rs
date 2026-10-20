use clap::Parser;
use wlitem::Item;

use crate::shared::real_path;
use crate::database::{ create_database_if_not_exists, update_or_add_item };
use calamine::{ Data, open_workbook, Ods, Xlsx, Reader };

#[derive(Parser)]
#[clap(version = "0.1.1", author = "Bryce Campbell <tonyhawk2100@gmail.com>", about = "import wishlist.", long_about = "import items to the wishlist.\r\n\r\nUnlike the export command, only two formats are supported, which are as follows:\r\n\r\n* JSON (imports item details and notes)\r\n* TSV (only imports item details).\r\n\r\nIf the imported data contains items that already exist, they will be updated.\r\n\r\nHowever, notes in a JSON file will not be updated and will be added as new notes.\r\n\r\nImports are done based on file extension, with TSV being the default.")]
pub struct Import {
    #[clap(default_value = "~/wishlist/gift_registry.db", help = "the path to the wishlist database")]
    pub file_path: String,

    #[clap(long, short, help = "the file to import data from.")]
    pub input_file: String
}

impl Import {
    pub fn run(&self) {
        create_database_if_not_exists(&self.file_path);
        let origin_path = real_path(&self.input_file);
        
        let items = match origin_path {
            ref p if p.ends_with(".json") => if let Ok(decoded_items) = Item::from_file(p) {
                decoded_items
            } else {
                vec![]
            },
            _ => if let Ok(decoded_items) = Item::from_tsv_file(&origin_path) {
                decoded_items
            } else {
                vec![]
            }
        };

        for item in items {
            update_or_add_item(&self.file_path, &item);
        }
    }
}

fn items_from_ods(p: &str) -> Vec<Item> {
    let mut items: Vec<Item> = vec![];
    let mut workbook: Ods<_> = open_workbook(p).expect("Could not read workbook");
    let wishlist = workbook.worksheet_range("Wishlist").expect("Wishlist sheet not found.");

    for (row_index, row) in wishlist.rows().enumerate() {
        if row_index == 0 {
            continue;
        } else {
            items.push(item_from_ods_row(&row));
        }
    }

    if let Ok(note_sheet) = workbook.worksheet_range("Notes") {
        let links = workbook.worksheet_range("Links").expect("Notes should be linked to items if provided.");
        let mut notes: Vec<String> = vec![];
        let associations = ods_note_associations(&workbook);

        for row in note_sheet.rows() {
            if let Some(data) = row.first {
                if let Data::String(note) = data {
                    notes.push(note);
                }
            }
        }

        for key in associations.keys() {
            if let Some(item) = items.iter().find(|i| i.id == key) {
                if let Some(note_ids) = associations.get(key) {
                    for note_id in note_ids {
                        item.notes.push(notes[note_id as usize]);
                    }
                }
            }
        }
    }

    items
}

fn item_from_ods_row(row: &[Data]) -> Item {
    let mut builder = Item::builder();

    for (column_index, data) in row.iter().enumerate() {
        match column_index {
            0 => if let Data::String(item_id) = data {
                builder.set_id(&item_id);
            },
            1 => if let Data::String(name) = data {
                builder.set_name(&name);
            },
            2 => if let Data::Int(quantity) = data {
                builder.set_quantity(quantity);
            },
            3 => if let Data::String(priority) = data {
                builder.set_priority(&priority);
            },
            4 => if let Data::String(url) = data {
                builder.set_url(&url);
            },
            _ => ()
        }
    }

    builder.build()
}

fn ods_note_associations(workbook: &Ods<_>) -> HashMap<String, Vec<Int>> {
    let link_sheet = workbook.worksheet_range("Links").expect("Could not open Links sheet.");
    let links = HashMap::new();

    for (index, row) in link_sheet.rows().iter().enumerate() {
        if index == 0 {
            continue;
        } else {
            let mut item_id = String::default();

            for (column_index, data) in row.iter().enumate() {
                match column_index {
                    0 => if Data::String(id) = data {
                        item_id = data;
                    },
                    1 => if Data::Int(note_id) = data {
                        if let Some(mut ids) = links.get_mut(item_id) {
                            ids.push(note_id);
                        } else {
                            links.insert(item_id, vec![note_id]);
                        }
                    },
                    _ => ()
                }
            }
        }
        
    }

    links
} 
