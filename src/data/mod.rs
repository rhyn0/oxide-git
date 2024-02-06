/// These functions and operations interact with the ogit database
pub mod filesystem;
pub mod objects;
pub mod prelude {
    pub use super::{filesystem, objects};
}
