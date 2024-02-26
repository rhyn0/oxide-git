use sha1::{Digest, Sha1};
use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OgitObjectType {
    Blob,
    #[allow(dead_code)]
    Tree,
    #[allow(dead_code)]
    Commit,
}

impl FromStr for OgitObjectType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blob" => Ok(Self::Blob),
            "tree" => Ok(Self::Tree),
            "commit" => Ok(Self::Commit),
            _ => Err("Invalid object type".to_string()),
        }
    }
}

impl Display for OgitObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Blob => "blob",
                Self::Tree => "tree",
                Self::Commit => "commit",
            }
        )
    }
}

/// Git creates objects of various types and allows references to them as args in exchangeable ways
#[derive(Debug, Clone)]
pub struct OgitObject {
    id: Vec<u8>,
    header: String,
    pub data: String,
    pub variant: OgitObjectType,
}

impl Display for OgitObject {
    /// Only want to display the ID of the object when pretty print
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.hex_string())
    }
}

impl OgitObject {
    pub fn new(data: &str, variant: OgitObjectType) -> Self {
        let mut hasher = Sha1::new();
        // add Git object header. More info: https://git-scm.com/book/en/v2/Git-Internals-Git-Objects
        let header = format!("{} {}\0", variant, data.len());
        let final_content = format!("{header}{data}");
        hasher.update(final_content.as_bytes());
        let hash_result = hasher.finalize().to_vec();
        Self {
            id: hash_result,
            header,
            data: data.to_string(),
            variant,
        }
    }
    /// Initialize object from object database
    pub fn from_database(content: &str) -> Self {
        let mut hasher = Sha1::new();
        hasher.update(content.as_bytes());
        let hash_result = hasher.finalize().to_vec();

        // have to read out the object type from content header
        let (header, data) = content.split_once('\0').unwrap();
        let (variant, _) = header.split_once(' ').unwrap();
        let mut header = header.to_string();
        header.push('\0');
        let variant = variant.parse().unwrap();

        Self {
            id: hash_result,
            data: data.to_string(),
            header,
            variant,
        }
    }
    pub fn hex_string(&self) -> String {
        self.id.iter().fold(String::new(), |mut acc, b| {
            acc.push_str(&format!("{b:02x}"));
            acc
        })
    }
    pub fn file_content(&self) -> String {
        format!("{}{}", self.header, self.data)
    }
    /// Outputs relative path to object in database
    ///
    /// Note that this is a relative path, and the actual path will be relative to the `.ogit` directory
    pub fn object_database_filepath(&self) -> String {
        let hex = self.hex_string();
        format!("{}/{}", &hex[..2], &hex[2..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    #[test]
    fn test_object_filepath() {
        let object = OgitObject::new("hello world", OgitObjectType::Blob);
        assert_eq!(
            object.object_database_filepath(),
            "95/d09f2b10159347eece71399a7e2e907ea3df4f"
        );
    }
    #[test]
    fn test_hash() {
        let object = OgitObject::new("hello world", OgitObjectType::Blob);
        assert_eq!(
            object.hex_string(),
            "95d09f2b10159347eece71399a7e2e907ea3df4f".to_string(),
        );
    }
    #[test]
    fn test_hashing() {
        let mut hasher = Sha1::new();
        hasher.update(b"hello world");
        let result = hasher.finalize();
        assert_eq!(result[..], hex!("2aae6c35c94fcfb415dbe95f408b9ce91ee846ed"));
    }
    #[test]
    fn test_hash_display() {
        let mut hasher = Sha1::new();
        hasher.update(b"hello world");
        let result = hasher.finalize();
        let display = format!("{:x}", result);
        assert_eq!(display, "2aae6c35c94fcfb415dbe95f408b9ce91ee846ed");
    }
    #[test]
    fn test_display() {
        let variants_with_expected = vec![
            (OgitObjectType::Blob, "blob"),
            (OgitObjectType::Tree, "tree"),
            (OgitObjectType::Commit, "commit"),
        ];
        for (variant, expected) in variants_with_expected {
            assert_eq!(format!("{variant}"), expected);
        }
    }
}
