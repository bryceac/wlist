use std::{ fs, path::{ Path, PathBuf }, collections::HashMap };

#[cfg(windows)]
use std::env;

use rusqlite::{ Connection, params };
use wlitem::Item;
use crate::{shared::*, note::Note };

pub fn copy_database_if_not_exists(p: &str) {
    let target = real_path(p);
    let destination_path = Path::new(&target);

    #[cfg(windows)]
    let original_path: PathBuf = if let Ok(path) = env::current_exe() {
        if let Some(db_directory) = path.parent() {
            db_directory.join("gift_registry.db")
        } else {
            Path::new("gift_registry.db").to_path_buf()  
        }
    } else {
        Path::new("gift_registry.db").to_path_buf()
    };

    #[cfg(unix)]
    let original_path: PathBuf = Path::new(&real_path("/var/db/wlist/gift_registry.db")).to_path_buf();
    

    if !destination_path.exists() {
        let _ = fs::create_dir_all(destination_path.parent().unwrap());
        let _ = fs::copy(original_path, destination_path);
    }
}

pub fn load_notes_from_db(p: &str) -> Vec<Note> {
    let mut notes: Vec<Note> = vec![];

    match Connection::open(&real_path(p)) {
        Ok(db) => {
            if let Ok(mut statement) = db.prepare("SELECT * FROM notes") {
                let note_query = statement.query_map([], |row| {
                    let id: u32 = if let Ok(num) = row.get(0) {
                        num
                    } else {
                        0
                    };

                    let note: String = row.get_unwrap(1);

                    Ok(Note::from(id, &note))
                }).unwrap();

                for note in note_query {
                    if let Ok(note) = note {
                        notes.push(note);
                    }
                }
            }
        },
        _ => {}
    }

    notes
}

fn note_with_id(p: &str, id: u32) -> Option<Note> {
    let notes = load_notes_from_db(p);

    if let Some(note_index) = notes.iter().position(|n| n.id == id) {
        Some(notes[note_index].clone())
    } else {
        None
    }
}

fn id_for_note(p: &str, note: &str) -> Option<u32> {
    let notes = load_notes_from_db(p);

    if let Some(note_index) = notes.iter().position(|n| n.note == note) {
        Some(notes[note_index].id)
    } else {
        None
    }
}

fn add_note(p: &str, note: &str) {
    if let Ok(db) = Connection::open(p) {
        let insert_note_statement = "Insert INTO notes(note) VALUES (?1)";

        if let Ok(mut statement) = db.prepare(insert_note_statement) {
            if let Err(error) = statement.execute([note]) {
                println!("{}", error);
            }
        }
    } else {}
}

fn link_note_to_item(p: &str, item: Item, note: &str) {
    let note_id = id_for_note(p, note).unwrap();

    if let Ok(db) = Connection::open(p) {
        let insert_link_statement = "INSERT INTO item_notes VALUES (?1, ?2)";

        if let Ok(mut statement) = db.prepare(insert_link_statement) {
            if let Err(error) = statement.execute(params![item.id, note_id]) {
                println!("{}", error);
            }
        }
    }
}

pub fn delete_note_with_id(p: &str, note_id: u32) {
    if let Ok(db) = Connection::open(p) {
        let delete_statement = "DELETE FROM notes WHERE id = (?1)";

        if let Ok(mut statement) = db.prepare(delete_statement) {
            if let Err(error) = statement.execute(params![note_id]) {
                println!("{}", error)
            }
        }
    }
}

fn remove_note_from_item(p: &str, item: Item, note_id: u32) {
    if let Ok(db) = Connection::open(p) {
        let remove_link_statement = "DELETE FROM item_notes WHERE item_id = (?1) AND note_id = (?2)";

        if let Ok(mut statement) = db.prepare(remove_link_statement) {
            if let Err(error) = statement.execute(params![item.id, note_id]) {
                println!("{}", error);
            }
        }
    }
}

fn retrieve_notes_for_item_with_id(p: &str, item_id: &str) -> Vec<String> {
    let mut item_notes: Vec<String> = vec![];

    match Connection::open(p) {
        Ok(db) => {
            let note_query = format!("SELECT note_id FROM item_notes WHERE item_id = '{}'", item_id);

            if let Ok(mut statement) = db.prepare(&note_query) {
                let note_id_query = statement.query_map([], |row| {
                    let note_id: u32 = row.get(0).expect("unable to parse value");

                    Ok(note_id)
                }).unwrap();

                for note_id in note_id_query {
                    if let Ok(note_id) = note_id {
                        if let Some(note) = note_with_id(p, note_id) {
                            item_notes.push(note.note);
                        }
                    }
                }
            }
        },
        _ => {}
    }

    item_notes
}

pub fn load_items_from_db(p: &str) -> Vec<Item> {
    let mut items: Vec<Item> = vec![];

    match Connection::open(&real_path(p)) {
        Ok(db) => {
            if let Ok(mut statement) = db.prepare("SELECT * FROM registry") {
                let item_query = statement.query_map([], |row| {
                    let id: String = row.get_unwrap(0);
                    let name: String = row.get_unwrap(1);
                    let quantity: u32 = if let Ok(num) = row.get(2) {
                        num
                    } else {
                        1
                    };

                    let priority: String = row.get_unwrap(3);
                    let url: String = row.get_unwrap(4);

                    let mut item = Item::builder();

                    item.set_id(&id)
                    .set_name(&name)
                    .set_quantity(quantity)
                    .set_priority(&priority)
                    .set_url(&url);

                    for note in retrieve_notes_for_item_with_id(p, &id) {
                        item.add_note(&note);
                    }

                    Ok(item.build())
                }).unwrap();

                for item in item_query {
                    if let Ok(item) = item {
                        items.push(item.clone());
                    }
                }
            }
        },
        _ => {}
    }

    items
}

pub fn item_with_id(p: &str, id: &str) -> Option<Item> {
    let items = load_items_from_db(p);

    if let Some(item_index) = items.iter().position(|item| item.id == id) {
        Some(items[item_index].clone())
    } else {
        None
    }
}

pub fn item_note_associations(p: &str) -> HashMap<u32, Vec<String>> {
    let mut associations: HashMap<u32, Vec<String>> = HashMap::new();

    if let Ok(db) = Connection::open(p) {

    }

    associations
}