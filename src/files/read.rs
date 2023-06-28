use std::fs;
use super::Path;


pub fn read_file(path: &Path) -> Option<String> {
    return match fs::read_to_string(path.clone()) {
        Ok(data) => Some(data),
        Err(_) => None,
    }
}
