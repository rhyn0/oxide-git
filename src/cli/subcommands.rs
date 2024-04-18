use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// Turn debugging and verbose information on to different levels
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize resources for `ogit`
    Init,
    /// Compute object ID and optionally creates a blob from a file.
    #[command(name = "hash-object")]
    HashObject { file: String },
    /// Read the content of the object with the given ID
    #[command(name = "cat-file")]
    CatObject { object_id: String },
    #[command(name = "write-tree")]
    WriteTree { directory: Option<String> },
    #[command(name = "read-tree", about = "Read a tree object into the index")]
    ReadTree { tree_id: String },
    #[command(
        name = "commit-tree",
        about = "Create a new commit object based on provided tree"
    )]
    CommitTree {
        tree_id: String,
        // there can be multiple parents, so this is Vec
        #[arg(short, help = "The parent commit object")]
        parent: Option<Vec<String>>,
        #[arg(short = 'm', help = "The commit message")]
        message: Option<String>,
    },
    #[command(
        name = "commit",
        about = "Creates a new commit as a high-level operation including HEAD and reflog side effects."
    )]
    Commit {
        /// currently implied that everything is in the index, so assume a '-a' flag is always present
        #[arg(short = 'm', long = "message", help = "The commit message")]
        message: Option<String>,
    },
    #[command(name = "log", about = "Show commit logs.")]
    Log {
        #[arg(help = "The commit object")]
        commit: Option<String>,
    },
    #[command(name = "checkout", about = "Checkout commit (for now) and move HEAD")]
    Checkout {
        #[arg(help = "The commit to checkout")]
        commit: String,
    },
    #[command(name = "tag", about = "Add a name to a specific commit")]
    Tag {
        #[arg(help = "Name of the tag")]
        tag_name: String,
        #[arg(help = "Object Id to be tagged")]
        oid: Option<String>,
    },
}
