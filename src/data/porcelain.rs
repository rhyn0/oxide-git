/// Collection of high-level functions for interacting with the database.
use crate::data::prelude::*;

use self::objects::OgitObject;

pub fn commit(message: Option<String>) -> Result<OgitObject, std::io::Error> {
    // for now let commit-tree handle the stdin message
    let tree = base::write_tree(None)?;
    let tree_id = tree.hex_string();
    let head_parent = filesystem::read_head_file()?.trim().to_owned();

    // porcelain commit is linear, so only one parent
    let parents: Vec<String> = if head_parent.is_empty() {
        vec![]
    } else {
        vec![head_parent]
    };
    let commit = base::commit_tree(&tree_id, &parents, message)?;
    filesystem::update_head_file(&commit)?;
    Ok(commit)
}
