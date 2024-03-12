use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
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
}
