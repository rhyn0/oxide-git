use sha1::{Digest, Sha1};
use std::fmt::Display;

/// Git creates objects of various types and allows references to them as args in exchangeable ways
#[derive(Debug, Clone)]
pub struct OgitObject {
    data: Vec<u8>,
    #[allow(dead_code)]
    variant: OgitObjectType,
}

#[derive(Debug, Clone)]
pub enum OgitObjectType {
    Blob,
    #[allow(dead_code)]
    Tree,
    #[allow(dead_code)]
    Commit,
}

impl ToString for OgitObjectType {
    fn to_string(&self) -> String {
        match self {
            Self::Blob => "blob".to_string(),
            Self::Tree => "tree".to_string(),
            Self::Commit => "commit".to_string(),
        }
    }
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
        hasher.update(format!("{} {}\0", variant.to_string(), data.len()).as_bytes());
        // add data body
        hasher.update(data.as_bytes());
        let hash_result = hasher.finalize().to_vec();
        Self {
            data: hash_result,
            variant,
        }
    }
    pub fn hex_string(&self) -> String {
        self.data.iter().fold(String::new(), |mut acc, b| {
            acc.push_str(&format!("{b:02x}"));
            acc
        })
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
    fn test_display() {
        let mut hasher = Sha1::new();
        hasher.update(b"hello world");
        let result = hasher.finalize();
        let display = format!("{:x}", result);
        assert_eq!(display, "2aae6c35c94fcfb415dbe95f408b9ce91ee846ed");
    }
}
