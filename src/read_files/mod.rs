use std::fs;


pub fn read_file(path: String) -> Option<String> {
    return match fs::read_to_string(path.clone()) {
        Ok(data) => Some(data),
        Err(_) => None,
    }
}
