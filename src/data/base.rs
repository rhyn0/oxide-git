use std::{fs, path::PathBuf};

/// Higher order operations on data.
use crate::data::objects::{OgitObject, OgitObjectType};

/// Recursively writes a directory to the ogit database.
///
/// An Ogit Tree is a collection of Ogit Blobs and Ogit Trees.
/// So find the sub items and create them as Ogit Blobs or Ogit Trees
/// and pass up the sub roots to the parent tree.
pub fn write_tree(directory: Option<PathBuf>) -> Result<OgitObject, std::io::Error> {
    // let mut tree = OgitObject::new(OgitObjectType::Tree);
    for entry in fs::read_dir(directory.unwrap_or_else(|| PathBuf::from(".")))? {
        // take it straight fromo the examples
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let sub_tree = write_tree(Some(path))?;
            println!("sub_tree: {sub_tree:?}");
        } else {
            println!("path: {path:?}");
        }
    }
    Ok(OgitObject::new("tree", OgitObjectType::Tree))
}
