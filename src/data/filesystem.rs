use itertools::Itertools;
use sha1::{Digest, Sha1};
use std::{
    env::current_dir,
    fs::{create_dir, write},
    path::PathBuf,
};

use super::objects::OgitObject;

const OGIT_DIR: &str = ".ogit";

/// Initializes the filesystem resources for ogit to operate
pub fn ogit_init() -> std::io::Result<()> {
    // fine here to panic if Err, don't have permission in this location list directory
    let mut new_ogit_dir = current_dir()?;
    new_ogit_dir.push(PathBuf::from(OGIT_DIR));
    let mut ogit_obj_database_dir = new_ogit_dir.clone();
    ogit_obj_database_dir.push("objects");

    // raise error around unable to create directory.
    create_dir(new_ogit_dir)?;
    create_dir(ogit_obj_database_dir)?;

    Ok(())
}

pub fn hash_object(data: &str) -> Result<OgitObject, std::io::Error> {
    let mut hasher = Sha1::new();
    hasher.update(data.as_bytes());
    let result = hasher.finalize();

    let object = OgitObject::new(format!("{result:x}"));
    // write object to database
    let mut object_path = match current_dir() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error getting current directory: {e}");
            return Err(e);
        }
    };
    object_path.push(PathBuf::from(OGIT_DIR.to_string()));
    object_path.push(PathBuf::from("objects"));

    // this result will return a 2 character directory and the rest of the hash
    let object_file_path = object.object_database_filepath();
    let (dir, file) = object_file_path.split('/').collect_tuple().unwrap();
    object_path.push(PathBuf::from(dir));
    // check if the directory for the object exists, if not create it
    if !object_path.exists() {
        match create_dir(object_path.clone()) {
            Ok(()) => (),
            Err(e) => {
                eprintln!(
                    "Error creating new Object directory: {}",
                    object_path.display()
                );
                return Err(e);
            }
        }
    }
    object_path.push(file);

    // write and catch the possible IO error
    write(object_path, data)?;
    Ok(object)
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;
    #[test]
    fn test_hashing() {
        let mut hasher = Sha1::new();
        hasher.update(b"hello world");
        let result = hasher.finalize();
        assert_eq!(result[..], hex!("2aae6c35c94fcfb415dbe95f408b9ce91ee846ed"));
    }
    #[test]
    fn test_display() {
        let mut hasher = Sha1::new();
        hasher.update(b"hello world");
        let result = hasher.finalize();
        let display = format!("{:x}", result);
        assert_eq!(display, "2aae6c35c94fcfb415dbe95f408b9ce91ee846ed");
    }
}
