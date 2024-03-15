use chrono::prelude::*;

pub fn get_current_local() -> DateTime<Local> {
    Local::now()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;

    #[test]
    fn test_getting_time() {
        let time = get_current_local();
        let duration = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let year = duration.as_secs() / (365 * 24 * 60 * 60);
        assert_eq!(time.year(), 1970 + year as i32);
    }
}
