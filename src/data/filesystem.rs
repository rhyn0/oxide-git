const OGIT_DIR: &str = ".ogit";
use std::fs::create_dir;

/// Initializes the filesystem resources for ogit to operate
pub fn ogit_init() -> std::io::Result<()> {
    create_dir(OGIT_DIR)?;
    Ok(())
}
