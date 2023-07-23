use super::{Path, Paths, File, write::make_or_save_config};
use std::{fs, path};


pub fn move_to_opened(file: &mut File) {
	let default_paths = Paths::default();

	let full_conf_path: Vec<&str> = file.conf_path.split('/').collect();
	let conf_name = full_conf_path.get(full_conf_path.len() - 1).unwrap();

	let mut new_opened_path = Path::from(default_paths.config_opened_files_path.clone());
	new_opened_path.push_str(conf_name);
	
	move_file(&file.conf_path, &default_paths.config_opened_files_path);

	file.conf_path = new_opened_path;
}


pub fn move_to_closed(file: &File) {
	let default_paths = Paths::default();

	make_or_save_config(&file.path, file.scroll);

	move_file(&file.conf_path, &default_paths.config_softhex_path);
}


fn move_file(from: &Path, to: &Path) {
	let file = path::Path::new(from);

	let mut move_to_path = String::from(to);
	move_to_path.push_str(file.file_name().unwrap().to_str().unwrap());

	fs::rename(from, move_to_path).unwrap();
}

