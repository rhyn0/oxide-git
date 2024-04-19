/// Collection of high-level functions for interacting with the database.
use crate::data::prelude::*;

use self::{
    commits::OgitCommit,
    objects::{OgitObject, OgitObjectType},
};

pub fn commit(message: Option<String>) -> Result<OgitObject, std::io::Error> {
    // for now let commit-tree handle the stdin message
    let tree = base::write_tree(None)?;
    let tree_id = tree.hex_string();
    let head_parent = filesystem::read_ref_file("HEAD")?.trim().to_owned();

    // porcelain commit is linear, so only one parent
    let parents: Vec<String> = if head_parent.is_empty() {
        vec![]
    } else {
        vec![head_parent]
    };
    let commit = base::commit_tree(&tree_id, &parents, message)?;
    filesystem::update_ref_file("HEAD", &commit)?;
    Ok(commit)
}

/// Read the tree identified by `commit` and move update HEAD.
pub fn checkout(commit: &str) {
    let tobe_set_commit = match OgitCommit::get(commit) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Error while fetching commit id {commit}: {e}");
            return;
        }
    };
    let tree_object =
        match filesystem::get_object(&tobe_set_commit.tree, Some(OgitObjectType::Tree)) {
            Ok(obj) => obj,
            Err(e) => {
                eprintln!(
                    "Error while readig tree of id {}: {e}",
                    &tobe_set_commit.tree
                );
                return;
            }
        };
    match base::read_tree(&tobe_set_commit.tree) {
        Ok(()) => (),
        Err(e) => {
            eprintln!("Error while writing tree to directory: {e}");
            return;
        }
    };
    match filesystem::update_ref_file("HEAD", &tree_object) {
        Ok(()) => (),
        Err(e) => eprintln!("Error while updating HEAD file: {e}"),
    }
}
