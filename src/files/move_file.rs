use super::{Path, Paths, File};
use std::{fs, path};


pub fn move_to_opened(file: &File) {
	let default_paths = Paths::default();
	
	move_file(&file.conf_path, &default_paths.config_opened_files_path);
}


pub fn move_to_closed(file: &File) {
	let default_paths = Paths::default();

	move_file(&file.conf_path, &default_paths.config_softhex_path);
}


fn move_file(from: &Path, to: &Path) {
	let file = path::Path::new(from);

	let mut move_to_path = String::from(to);
	move_to_path.push_str(file.file_name().unwrap().to_str().unwrap());

	fs::rename(from, move_to_path).unwrap();
}

