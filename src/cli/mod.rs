/// Code is divided into understanding the logic of a command
pub mod functions;
mod subcommands;
pub use subcommands::{Cli, Commands};

pub mod prelude {
    pub use super::{functions, Cli, Commands};
    pub use clap::Parser;
}
