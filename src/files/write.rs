use std::{fs, io::Write, path};
use super::{Path, File};
use uid::Id as IdT;


const CONFIG_EXTENSION: &str = ".conf";
const CONFIG_PATH: &str = "$HOME/.config/";
const CONFIG_SOFTHEX_PATH: &str = "$HOME/.config/softhex/";
const CONFIG_FILE_PATH: &str = "$HOME/.config/softhex/opened_files.conf/";
const CONFIG_OPENED_FILES_PATH: &str = "$HOME/.config/softhex/opened_files/";


struct T(());
type Uid = IdT<T>;


fn write_in_file(path: &Path, data: &String) {
	let mut file = fs::File::create(path).unwrap();

	file.write_all(data.as_bytes());
}


fn append_data_in_file(path: &Path, data: &String) {
	let mut file = fs::OpenOptions::new()
		.write(true)
		.append(true)
		.open(path)
		.unwrap();

	file.write_all(data.as_bytes());
}


fn make_config_file_if_not_exist() {
	let path = path::Path::new(CONFIG_PATH);

	if !path.exists() {
		fs::create_dir(CONFIG_PATH);
	}

	let path = path::Path::new(CONFIG_SOFTHEX_PATH);

	if !path.exists() {
		fs::create_dir(CONFIG_SOFTHEX_PATH);
	}

	let conf_file = path::Path::new(CONFIG_FILE_PATH);

	if !conf_file.exists() {
		write_in_file(&CONFIG_FILE_PATH.to_string(), &"".to_string());
	}

	let path = path::Path::new(CONFIG_OPENED_FILES_PATH);

	if !path.exists() {
		fs::create_dir(CONFIG_OPENED_FILES_PATH);
	}
}


pub fn make_or_save_config(path: &Path, file: &File) {
	make_config_file_if_not_exist();

	let uid = Uid::new();

	let mut base_config_line = String::from(&uid.get().to_string());
	base_config_line.push('=');
	base_config_line.push_str(&file.path);
	base_config_line.push('\n');

	append_data_in_file(&CONFIG_FILE_PATH.to_string(), &base_config_line);

	let mut config_file_path = String::from(CONFIG_OPENED_FILES_PATH);
	config_file_path.push_str(&uid.get().to_string());
	config_file_path.push_str(CONFIG_EXTENSION);
	
	let mut buffer = String::from("path=");
	buffer.push_str(&file.data);
	buffer.push_str("\nscroll=");
	buffer.push_str(&file.scroll.to_string());
	buffer.push_str("\nline_counter=");
	buffer.push_str(&file.line_counter.to_string());
	// TODO: 7

	write_in_file(&config_file_path, &buffer);
}

