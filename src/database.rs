use std::{ fs, path::{ Path, PathBuf }, collections::HashMap };

#[cfg(windows)]
use std::env;

use rusqlite::{ Connection, params };
use wlitem::{Item, Priority};
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

fn link_note_to_item(p: &str, item: &Item, note: &str) {
    let note_id = if let Some(id) = id_for_note(p, note) {
        id
    } else {
        add_note(p, note);
        id_for_note(p, note).unwrap()
    };

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
            } else {
                delete_item_note_associations(p, None, Some(note_id));
            }
        }
    }
}

pub fn update_note_with_id(p: &str, note_id: u32, note: &str) {
    if let Ok(db) = Connection::open(p) {
        let update_statement = "UPDATES notes SET note = ?1 WHERE id = ?2";

        if let Ok(mut statement) = db.prepare(update_statement) {
            if let Err(error) = statement.execute(params![note, note_id]) {
                println!("{}", error);
            }
        }
    }
}

pub fn remove_note_from_item(p: &str, item: Item, note_id: u32) {
    if let Ok(db) = Connection::open(p) {
        let remove_link_statement = "DELETE FROM item_notes WHERE item_id = (?1) AND note_id = (?2)";

        if let Ok(mut statement) = db.prepare(remove_link_statement) {
            if let Err(error) = statement.execute(params![item.id, note_id]) {
                println!("{}", error);
            }
        }
    }
}

fn delete_item_note_associations(p: &str, item_id: Option<&str>, note_id: Option<u32>) {
    if item_id.is_some() && note_id.is_some() {
        println!("It is not necessary for both an item and note id to be provided. Please provide only one.");
        return;
    } else if item_id.is_none() && note_id.is_none() {
        println!("Please provide either an item or note id.");
        return;
    }

    let delete_statement = if item_id.is_some() {
        "DELETE FROM item_notes WHERE item_id = ?1"
    } else {
        "DELETE FROM item_notes WHERE note_id = ?1"
    };

    if let Ok(db) = Connection::open(p) {
        if let Ok(mut statement) = db.prepare(delete_statement) {
            if let Some(item_id) = item_id {
                if let Err(error) = statement.execute(params![item_id]) {
                    println!("{}", error)
                }
            } else {
                if let Err(error) = statement.execute(params![note_id.unwrap()]) {
                    println!("{}", error)
                }
            }
        }
    }
}

pub fn retrieve_notes_for_item_with_id(p: &str, item_id: &str) -> Vec<String> {
    let mut item_notes: Vec<String> = vec![];

    let note_relations = item_note_associations(p);

    if let Some(note_ids) = note_relations.get(item_id).cloned() {
        for id in note_ids {
            if let Some(note) = note_with_id(p, id) {
                item_notes.push(note.note);
            }
        }
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

    if let Some(item_index) = items.iter().position(|item| item.id.to_lowercase() == id.to_lowercase()) {
        Some(items[item_index].clone())
    } else {
        None
    }
}

pub fn item_note_associations(p: &str) -> HashMap<String, Vec<u32>> {
    let mut associations: HashMap<String, Vec<u32>> = HashMap::new();

    if let Ok(db) = Connection::open(p) {
        if let Ok(mut statement) = db.prepare("SELECT * FROM item_notes") {
            let item_note_query = statement.query_map([], |row| {
                let item_id: String = row.get_unwrap(0);
                let note_id: u32 = if let Ok(num) = row.get(1) {
                    num
                } else {
                    0
                };

                Ok((item_id,note_id))
            }).unwrap();

            for item_pair in item_note_query {
                let keys: Vec<String> = associations.keys().map(|key| key.to_owned()).collect();

                if let Ok(item_pair) = item_pair {
                    if keys.contains(&item_pair.0) {
                        if let Some(note_ids) = associations.get_mut(&item_pair.0) {
                            note_ids.push(item_pair.1);
                        } else {
                            associations.insert(item_pair.0, vec![item_pair.1]);
                        }
                    }
                }
            }
        }
    }

    associations
}

fn id_for_priority(p: &str, priority: &Priority) -> u32 {
    let mut id: u32 = 0;

    if let Ok(db) = Connection::open(p) {
        let priority_statement = format!("SELECT id FROM priorities WHERE id = '{}'", priority.to_str());

        if let Ok(mut statement) = db.prepare(&priority_statement) {
            if let Ok(priority_id) = statement.query_one([], |row| {
                row.get(0)
            }) {
                id = priority_id;
            }
        }
    }

    id
}

pub fn add_item(p: &str, item: Item) {
    let item_url = if let Some(url) = item.url.clone() {
        url.as_str().to_owned()
    } else {
        "".to_owned()
    };

    if let Ok(db) = Connection::open(p) {
        let insert_statement = "INSERT INTO items VALUES (?1, ?2, ?3, ?4, ?5)";

        if let Ok(mut statement) = db.prepare(insert_statement) {
            if let Err(error) = statement.execute(params![item.id, item.name, item.quantity, id_for_priority(p, &item.priority), item_url]) {
                println!("{}", error);
            } else {
                for note in item.notes.clone() {
                    link_note_to_item(p, &item, &note);
                }
            }
        }
    }
}

pub fn delete_item(p: &str, item: &Item) {
    if let Ok(db) = Connection::open(p) {
        let delete_statement = "DELETE FROM items WHERE id = ?1";

        if let Ok(mut statement) = db.prepare(delete_statement) {
            if let Err(error) = statement.execute([item.id.clone()]) {
                println!("{}", error);
            } else {
                delete_item_note_associations(p, Some(&item.id), None);
            }
        }
    }
}

pub fn update_item(p: &str, item: &Item) {

}