use super::Path;
use std::{fs, path};


pub fn move_file(from: &Path, to: &Path) {
	let file = path::Path::new(from);

	let mut move_to_path = String::from(to);
	move_to_path.push_str(file.file_name().unwrap().to_str().unwrap());

	fs::rename(from, move_to_path);
}

