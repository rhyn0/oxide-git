use crate::data::prelude::*;

use self::filesystem::ogit_init;

pub fn init() {
    let dir_created = ogit_init();
    if dir_created.is_err() {
        eprintln!("Failed to initialize ogit directory");
    };
}
