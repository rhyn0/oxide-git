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
        Commands::HashObject { file } => functions::hash_object_cmd(file),
        Commands::CatObject { object_id } => functions::cat_object_cmd(&object_id),
    };
}
