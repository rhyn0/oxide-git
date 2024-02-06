use std::fmt::Display;

/// Git creates objects of various types and allows references to them as args in exchangeable ways
#[derive(Debug, Clone)]
pub struct OgitObject {
    id: String,
}

impl Display for OgitObject {
    /// Only want to display the ID of the object when pretty print
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl OgitObject {
    pub const fn new(id: String) -> Self {
        Self { id }
    }
    /// Outputs relative path to object in database
    ///
    /// Note that this is a relative path, and the actual path will be relative to the `.ogit` directory
    pub fn object_database_filepath(&self) -> String {
        format!("{}/{}", &self.id[..2], &self.id[2..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_filepath() {
        let object = OgitObject::new("2aae6c35c94fcfb415dbe95f408b9ce91ee846ed".to_string());
        assert_eq!(
            object.object_database_filepath(),
            "2a/ae6c35c94fcfb415dbe95f408b9ce91ee846ed"
        );
    }
}
