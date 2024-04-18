mod cli;
mod data;

use cli::prelude::*;

fn main() {
    let cli = Cli::parse();
    match cli.debug {
        0 => eprint!(""),
        1 => eprintln!("Some debug info is displayed"),
        2 => eprintln!("All debug info is displayed"),
        _ => eprintln!("Are you in need of this much information?"),
    };
    match cli.command {
        Commands::Init => functions::init_cmd(),
        Commands::HashObject { file } => functions::hash_object_cmd(&file),
        Commands::CatObject { object_id } => functions::cat_object_cmd(&object_id),
        Commands::WriteTree { directory } => functions::write_tree_cmd(directory.as_deref()),
        Commands::ReadTree { tree_id } => functions::read_tree_cmd(&tree_id),
        Commands::CommitTree {
            tree_id,
            parent,
            message,
        } => {
            functions::commit_tree_cmd(&tree_id, parent.as_deref(), message);
        }
        Commands::Commit { message } => functions::commit_cmd(message),
        Commands::Log {} => functions::log_cmd(),
    };
}
