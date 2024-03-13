use itertools::Itertools;
use regex::Regex;
use std::{
    fmt::{Display, Write},
    fs, iter,
    path::{Path, PathBuf},
};

/// Higher order operations on data.
use crate::data::{
    filesystem::{self, hash_object},
    objects::{OgitObject, OgitObjectType},
};

#[derive(Debug, Clone)]
struct TreeEntry {
    filemode: String,
    filename: String,
    id: String,
    variant: OgitObjectType,
}
impl Display for TreeEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.filemode, self.variant, self.id, self.filename
        )
    }
}
impl TreeEntry {
    fn new(file_path: &Path, object: OgitObject) -> Self {
        Self {
            filemode: Self::format_filemode(
                filesystem::get_filemode(file_path).unwrap(),
                &object.variant,
            ),
            filename: filesystem::get_filename(file_path).unwrap().to_string(),
            id: object.hex_string(),
            variant: object.variant,
        }
    }
    fn format_filemode(filemode: u32, variant: &OgitObjectType) -> String {
        if variant == &OgitObjectType::Tree {
            "040000".to_string()
        } else {
            format!("{filemode:0>6o}")
        }
    }
}

/// Recursively writes a directory to the ogit database.
///
/// An Ogit Tree is a collection of Ogit Blobs and Ogit Trees.
/// So find the sub items and create them as Ogit Blobs or Ogit Trees
/// and pass up the sub roots to the parent tree.
#[allow(clippy::trivial_regex)]
pub fn write_tree(directory: Option<PathBuf>) -> Result<OgitObject, std::io::Error> {
    // let mut tree = OgitObject::new(OgitObjectType::Tree);
    let mut tree_entries = Vec::new();
    let ignore_lines = filesystem::load_ignore_file();
    // TODO: fix bug where `target/` won't match `target` as a path
    // Follow gitignore spec better: https://git-scm.com/docs/gitignore
    let ignore_regexes = ignore_lines
        .iter()
        .map(|line| {
            let line = line.replace('.', r"\.").replace('*', ".*");
            Regex::new(&line).unwrap()
        })
        // TODO: change this regex to be more specific
        .chain(iter::once(Regex::new(r"\.ogit").unwrap()))
        .collect_vec();
    for entry in fs::read_dir(directory.unwrap_or_else(|| PathBuf::from(".")))? {
        // take it straight fromo the examples
        let entry = entry?;
        let path = entry.path();
        if is_ignored(&path, &ignore_regexes) {
            continue;
        }
        if path.is_dir() {
            let sub_tree = write_tree(Some(path.clone()))?;
            tree_entries.push(TreeEntry::new(&path, sub_tree));
        } else {
            let data = fs::read(&path).unwrap();
            let object = hash_object(&data, Some(OgitObjectType::Blob)).unwrap();
            tree_entries.push(TreeEntry::new(&path, object));
        }
    }
    let tree_data = build_tree_data(&tree_entries);
    hash_object(tree_data.as_bytes(), Some(OgitObjectType::Tree))
}

fn build_tree_data(entries: &[TreeEntry]) -> String {
    entries.iter().fold(String::new(), |mut acc, entry| {
        writeln!(&mut acc, "{entry}").unwrap();
        acc
    })
}

/// Returns whether a file is ignored or not.
pub fn is_ignored(path: &Path, ignores: &[Regex]) -> bool {
    let result = ignores
        .iter()
        .any(|pattern| pattern.is_match(path.to_str().unwrap()));
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_entry_display_file() {
        // funny object so we don't have to worry about changing contents
        let test_object = OgitObject::new(b"hello world", OgitObjectType::Blob);
        let entry = TreeEntry::new(Path::new("src/data/objects.rs"), test_object.clone());
        assert_eq!(entry.filemode.len(), 6);
        assert_eq!(entry.filemode, "100644");
        assert_eq!(entry.filename, "objects.rs");
        assert_eq!(entry.id, test_object.hex_string());
    }
    #[test]
    fn test_tree_entry_display_folder() {
        // funny object so we don't have to worry about changing contents
        let test_object = OgitObject::new(b"hello world", OgitObjectType::Tree);
        let entry = TreeEntry::new(Path::new("src/data"), test_object.clone());
        assert_eq!(entry.filemode.len(), 6);
        assert_eq!(entry.filemode, "040000");
        assert_eq!(entry.filename, "data");
        assert_eq!(entry.id, test_object.hex_string());
    }
}
