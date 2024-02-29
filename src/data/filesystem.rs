use flate2::Compression;
use flate2::{read::ZlibDecoder, write::ZlibEncoder};
use itertools::Itertools;
use std::io::prelude::*;
use std::{
    env::current_dir,
    fs::{create_dir, write},
    path::PathBuf,
};

use super::objects::{OgitObject, OgitObjectType};

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

pub fn hash_object(
    data: &str,
    object_type: Option<OgitObjectType>,
) -> Result<OgitObject, std::io::Error> {
    let object = OgitObject::new(data.as_bytes(), object_type.unwrap_or(OgitObjectType::Blob));
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
    // compress the data before writing to the file
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(object.file_content().as_bytes())?;
    let compressed_bytes = encoder.finish().unwrap();
    // write the compressed data to the file
    write(object_path, compressed_bytes)?;
    Ok(object)
}

pub fn get_object(
    object_id: &str,
    expected_object_type: Option<OgitObjectType>,
) -> Result<OgitObject, std::io::Error> {
    let mut object_path = match current_dir() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error getting current directory: {e}");
            return Err(e);
        }
    };
    object_path.push(PathBuf::from(OGIT_DIR.to_string()));
    object_path.push(PathBuf::from("objects"));

    let (dir, file) = object_id.split_at(2);
    object_path.push(PathBuf::from(dir));
    object_path.push(PathBuf::from(file));

    let object_data = match std::fs::read(object_path) {
        Ok(data) => {
            let mut output = Vec::new();
            let mut decoder = ZlibDecoder::new(data.as_slice());
            let _ = decoder.read(&mut output).unwrap();
            output
        }
        Err(e) => {
            eprintln!("Error reading object file: {e}");
            return Err(e);
        }
    };
    let object = OgitObject::from_database(&object_data);
    if let Some(ogit_type) = expected_object_type {
        if object.variant != ogit_type {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Object type mismatch, expected: {ogit_type}, got: {}",
                    object.variant
                ),
            ));
        }
    }
    Ok(object)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_object() {
        let _ = ogit_init();
        let object = hash_object("hello world", None).unwrap();
        assert_eq!(object.variant, OgitObjectType::Blob);
        assert_eq!(object.data, "hello world".as_bytes());
        assert_eq!(object.file_content(), "blob 11\0hello world");
        let hex_string = object.hex_string();
        let path = object.object_database_filepath();
        assert_eq!(path, format!("{}/{}", &hex_string[..2], &hex_string[2..]));
        let mut object_path = PathBuf::from(OGIT_DIR.to_string());
        object_path.push(PathBuf::from("objects"));
        let (dir, file) = path.split('/').collect_tuple().unwrap();
        object_path.push(PathBuf::from(dir));
        object_path.push(PathBuf::from(file));
        assert!(object_path.exists());

        std::fs::remove_file(object_path).unwrap();
    }
}
