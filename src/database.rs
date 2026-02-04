use std::{ fs, path::{ Path, PathBuf } };

#[cfg(windows)]
use std::env;

use rusqlite::{ Connection, params };
use wlitem::Item;
use crate::shared::*;

pub fn copy_database_if_not_exists(p: &str) {
    let target = real_path(p);
    let destination_path = Path::new(&target);

    #[cfg(windows)]
    let original_path: PathBuf = if let Ok(path) = env::current_exe() {
        if let Some(db_directory) = path.parent() {
            db_directory.join("register.db")
        } else {
            Path::new("register.db").to_path_buf()  
        }
    } else {
        Path::new("register.db").to_path_buf()
    };

    #[cfg(unix)]
    let original_path: PathBuf = Path::new(&real_path("/var/db/rcheckbook/register.db")).to_path_buf();
    

    if !destination_path.exists() {
        let _ = fs::create_dir_all(destination_path.parent().unwrap());
        let _ = fs::copy(original_path, destination_path);
    }
}