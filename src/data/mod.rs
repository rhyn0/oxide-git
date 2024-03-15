/// These functions and operations interact with the ogit database
pub mod base;
pub mod config;
pub mod filesystem;
pub mod objects;
pub mod time;
pub mod prelude {
    pub use super::{base, config, filesystem, objects, time};
}
