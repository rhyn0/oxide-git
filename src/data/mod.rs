/// These functions and operations interact with the ogit database
pub mod base;
pub mod commits;
pub mod config;
pub mod filesystem;
pub mod objects;
pub mod porcelain;
pub mod time;

pub mod prelude {
    pub use super::{base, commits, config, filesystem, objects, porcelain, time};
}
