use std::fs;

use crate::data::prelude::*;

use self::filesystem::{hash_object, ogit_init};

pub fn init_cmd() {
    let dir_created = ogit_init();
    if dir_created.is_err() {
        eprintln!("Failed to initialize ogit directory");
    };
}

pub fn hash_object_cmd(file_path: String) {
    let file = match fs::read_to_string(file_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error reading content file: {e}");
            return;
        }
    };
    let object = hash_object(&file, None);
    match object {
        Ok(obj) => println!("{obj}"),
        Err(e) => eprintln!("Error: {e}"),
    }
}
