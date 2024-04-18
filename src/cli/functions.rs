use std::{fs, io::Write, path::Path};

use crate::data::prelude::*;

use self::{
    base::write_tree,
    commits::OgitCommit,
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

pub fn read_tree_cmd(tree_id: &str) {
    let tree = base::read_tree(tree_id);
    match tree {
        Ok(()) => (),
        Err(e) => eprintln!("Error reading tree: {e}"),
    }
}

pub fn commit_tree_cmd(tree_id: &str, parents: Option<&[String]>, message: Option<String>) {
    let commit = base::commit_tree(tree_id, parents.unwrap_or_default(), message);
    match commit {
        Ok(c) => println!("{c}"),
        Err(e) => eprintln!("Error: {e}. Aborting commit-tree."),
    }
}

pub fn commit_cmd(message: Option<String>) {
    let commit = porcelain::commit(message);
    match commit {
        Ok(c) => println!("{c}"),
        Err(e) => eprintln!("Error: {e}. Aborting commit."),
    }
}

pub fn log_cmd(commit: Option<String>) {
    let mut curr_head_id = if let Some(commit) = commit {
        commit
    } else {
        match filesystem::read_head_file() {
            Ok(h) => h,
            Err(e) => {
                eprintln!("Error reading HEAD file: {e}");
                return;
            }
        }
    };
    loop {
        let commit = match OgitCommit::get(&curr_head_id) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error reading commit: {e}");
                return;
            }
        };
        println!("commit {curr_head_id}");
        // could be something bad here about using println. such as formatting and other such should be handled by the OgitCommit
        println!("{commit}");
        // TODO: this index can be controlled by options passed on the command line
        // Check out --first-parent
        // https://git-scm.com/docs/git-log#Documentation/git-log.txt---first-parent
        curr_head_id = match commit.parents.get(0) {
            Some(p) => p.clone(),
            None => break,
        };
    }
}

pub fn tag_cmd(name: &str, oid: Option<String>) {
    let head = filesystem::read_head_file().ok();
    // yah this one needs work
    // TODO: fix this weird error work around
    let object_id = oid.unwrap_or_else(|| head.unwrap());
    base::create_tag(name, &object_id);
}
