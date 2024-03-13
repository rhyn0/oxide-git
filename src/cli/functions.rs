use std::{fs, io::Write, path::Path};

use crate::data::prelude::*;

use self::{
    base::write_tree,
    filesystem::{get_object, hash_object, ogit_init},
};

pub fn init_cmd() {
    let dir_created = ogit_init();
    if dir_created.is_err() {
        eprintln!("Failed to initialize ogit directory");
    };
}

pub fn hash_object_cmd(file_path: &str) {
    let file_content = match filesystem::read_file(Path::new(file_path)) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error reading content file: {e}");
            return;
        }
    };
    let object = hash_object(&file_content, None);
    match object {
        Ok(obj) => println!("{obj}"),
        Err(e) => eprintln!("Error: {e}"),
    }
}

pub fn cat_object_cmd(object_id: &str) {
    let object = get_object(object_id, None);
    match object {
        Ok(obj) => {
            let stdout = std::io::stdout();
            let mut handle = stdout.lock();
            handle.write_all(&obj.data).unwrap();
        }
        Err(e) => eprintln!("Error: {e}"),
    }
}

pub fn write_tree_cmd(directory: Option<&str>) {
    let directory = directory.map(|s| fs::canonicalize(s).unwrap());
    let tree = write_tree(directory);
    match tree {
        Ok(t) => println!("{t}"),
        Err(e) => eprintln!("Error: {e}"),
    }
}
