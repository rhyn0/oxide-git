use super::{filesystem::get_object, objects::OgitObjectType, time};

/// This is a helper object for printing out the commit object
/// Everything should be optimized to make displaying this object as easy as possible
#[derive(Debug, Clone)]
pub struct OgitCommit {
    pub tree: String,
    pub parents: Vec<String>,
    pub author: String,
    pub author_time: time::DateTime<time::FixedOffset>,
    pub committer: String,
    pub committer_time: time::DateTime<time::FixedOffset>,
    pub message: Vec<u8>, // message can be any byte sequence (emojis or such)
}

impl std::fmt::Display for OgitCommit {
    /// Pretty print the commit object, similar to git log does
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Author: {}", self.author)?;
        writeln!(f, "Date:   {}", self.author_time.format("%c %z"))?;
        writeln!(
            f,
            "\n    {}",
            std::str::from_utf8(&self.message).unwrap().trim_end()
        )
    }
}

impl OgitCommit {
    pub fn new(
        tree: String,
        parents: Vec<String>,
        author: &str,
        committer: &str,
        message: Vec<u8>,
    ) -> Self {
        let author_time = Self::get_epoch_offset_from_line(author);
        let author = author.split_whitespace().next().unwrap().to_string();
        let committer_time = Self::get_epoch_offset_from_line(committer);
        let committer = committer.split_whitespace().next().unwrap().to_string();
        Self {
            tree,
            parents,
            author,
            author_time,
            committer,
            committer_time,
            message,
        }
    }

    /// Parse a line of format:
    ///     'prefix item item EPOCH OFFSET'
    /// Into a `DateTime` object. Works best for the author and commiter lines
    fn get_epoch_offset_from_line(line: &str) -> time::DateTime<time::FixedOffset> {
        let line_parts = line.split_whitespace().collect::<Vec<&str>>();

        time::DateTime::parse_from_str(&line_parts[3..=4].join(" "), "%s %z").unwrap()
    }

    pub fn get(id: &str) -> std::io::Result<Self> {
        let object = get_object(id, Some(OgitObjectType::Commit))?;
        let data = object.data;
        let mut tree = String::new();
        let mut parents = Vec::new();
        let mut author = String::new();
        let mut committer = String::new();
        let mut message = Vec::new();
        let mut lines = data.split(|b| *b == b'\n');
        for line in &mut lines {
            if line.starts_with(b"tree ") {
                let line = std::str::from_utf8(line).unwrap();
                // we know the prefix is 5 bytes long
                let (_, tree_id) = line.split_at(5);
                tree = tree_id.to_string();
            } else if line.starts_with(b"parent ") {
                let parent = std::str::from_utf8(line).unwrap();
                // prefix is 7 bytes long
                let (_, parent_id) = parent.split_at(7);
                parents.push(parent_id.to_string());
            } else if line.starts_with(b"author ") {
                let author_line = std::str::from_utf8(line).unwrap();
                // prefix is 7 bytes long
                let (_, author_id) = author_line.split_at(7);
                author = author_id.to_string();
            } else if line.starts_with(b"committer ") {
                let commiter_line = std::str::from_utf8(line).unwrap();
                // prefix is 10 bytes long
                let (_, commiter_id) = commiter_line.split_at(10);
                committer = commiter_id.to_string();
            } else {
                // handle the message line and exit
                message = lines.collect::<Vec<&[u8]>>().join(&b'\n');
                break;
            }
        }
        Ok(Self::new(tree, parents, &author, &committer, message))
    }
}
