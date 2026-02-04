use std::{ fs, path::{ Path, PathBuf } };

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