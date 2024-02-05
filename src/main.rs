mod cli;
mod data;

use cli::prelude::*;

fn main() {
    let cli = Cli::parse();
    match cli.debug {
        0 => eprintln!("Debug info is off"),
        1 => eprintln!("Some debug info is displayed"),
        2 => eprintln!("All debug info is displayed"),
        _ => eprintln!("Are you in need of this much information?"),
    };
    match cli.command {
        Commands::Init => functions::init(),
    };
}
