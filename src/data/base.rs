use itertools::Itertools;
use regex::Regex;
use std::{
    fmt::{Display, Write},
    fs, iter,
    path::{Path, PathBuf},
    str::FromStr,
};

/// Higher order operations on data.
use crate::data::{
    filesystem::{self, hash_object},
    objects::{OgitObject, OgitObjectType},
};

#[derive(Debug, Clone)]
struct TreeEntry {
    filemode: String,
    filename: PathBuf,
    id: String,
    variant: OgitObjectType,
}
impl Display for TreeEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.filemode,
            self.variant,
            self.id,
            self.filename.file_name().unwrap().to_str().unwrap()
        )
    }
}
impl FromStr for TreeEntry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let filemode = parts.next().unwrap();
        let variant = parts.next().unwrap();
        let id = parts.next().unwrap();
        // should only be a singular item here
        let filename = parts.next().unwrap();
        Ok(Self {
            filemode: filemode.to_string(),
            filename: PathBuf::from(filename),
            id: id.to_string(),
            variant: OgitObjectType::from_str(variant).unwrap(),
        })
    }
}
impl TreeEntry {
    fn new(file_path: &Path, object: OgitObject) -> Self {
        Self {
            filemode: Self::format_filemode(
                filesystem::get_filemode(file_path).unwrap(),
                &object.variant,
            ),
            filename: PathBuf::from(filesystem::get_filename(file_path).unwrap()),
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
            let object = hash_object(&data, Some(OgitObjectType::Blob))?;
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

/// Returns iterator of the `TreeEntry` in the tree object specified by `tree_id`.
fn parse_tree_data(tree_id: Option<&str>) -> Vec<TreeEntry> {
    let tree = if let Some(id) = tree_id {
        filesystem::get_object(id, Some(OgitObjectType::Tree)).unwrap()
    } else {
        return Vec::new();
    };
    // OgitObject stores file content as bytes, but since this is tree we know its valid UTF8
    let tree_data = String::from_utf8(tree.data).unwrap();
    tree_data
        .lines()
        .map(|line| line.parse::<TreeEntry>().unwrap())
        .collect_vec()
}

/// Returns path to object and the OID of its contents
fn get_tree(tree_id: &str, path: Option<PathBuf>) -> Result<Vec<TreeEntry>, std::io::Error> {
    let base_path = path.unwrap_or_else(|| PathBuf::from("."));
    let mut result = Vec::new();
    let mut entries = parse_tree_data(Some(tree_id));
    for entry in &mut entries {
        let mut path = base_path.clone();
        match entry.variant {
            OgitObjectType::Blob => {
                path.push(&entry.filename);
                entry.filename = path;
                result.push(entry.clone());
            }
            OgitObjectType::Tree => {
                path.push(&entry.filename);
                let sub_tree = get_tree(&entry.id, Some(path))?;
                result.extend(sub_tree);
            }
            OgitObjectType::Commit => panic!("Invalid object type in tree"),
        }
    }
    Ok(result)
}

pub fn read_tree(tree_id: &str) -> Result<(), std::io::Error> {
    let entries = get_tree(tree_id, None)?;
    for entry in entries {
        println!("{entry}");
    }
    Ok(())
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
        assert_eq!(entry.filename, PathBuf::from("objects.rs"));
        assert_eq!(entry.id, test_object.hex_string());
    }
    #[test]
    fn test_tree_entry_display_folder() {
        // funny object so we don't have to worry about changing contents
        let test_object = OgitObject::new(b"hello world", OgitObjectType::Tree);
        let entry = TreeEntry::new(Path::new("src/data"), test_object.clone());
        assert_eq!(entry.filemode.len(), 6);
        assert_eq!(entry.filemode, "040000");
        assert_eq!(entry.filename, PathBuf::from("data"));
        assert_eq!(entry.id, test_object.hex_string());
    }
}
