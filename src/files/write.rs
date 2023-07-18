use std::{fs, io::Write, path};
use super::{Path, File, Paths, CONFIG_EXTENSION, read::read_file};
use uid::Id as IdT;


const UID_BEFORE_EQ: usize = 0_usize;
const PATH_AFTER_EQ: usize = 1_usize;
pub const PATH_IN_CONFIG_AT_VEC: usize = 0_usize;
pub const SCROLL_IN_CONFIG_AT_VEC: usize = 1_usize;


struct T(());
type Uid = IdT<T>;


fn write_in_file(path: &Path, data: &String) {
	let mut file = fs::File::create(path).unwrap();

	file.write_all(data.as_bytes()).unwrap();
}


fn append_data_in_file(path: &Path, data: &String) {
	let mut file = fs::OpenOptions::new()
		.write(true)
		.append(true)
		.open(path)
		.unwrap();

	file.write_all(data.as_bytes()).unwrap();
}


pub fn make_config_file_if_not_exist(paths: &Paths) {
	let path = path::Path::new(&paths.config_path);

	if !path.exists() {
		fs::create_dir(&paths.config_path).unwrap();
	}

	let path = path::Path::new(&paths.config_softhex_path);

	if !path.exists() {
		fs::create_dir(&paths.config_softhex_path).unwrap();
	}

	let conf_file = path::Path::new(&paths.config_file_path);

	if !conf_file.exists() {
		fs::File::create(&paths.config_file_path).unwrap();
	}

	let path = path::Path::new(&paths.config_opened_files_path);

	if !path.exists() {
		fs::create_dir(&paths.config_opened_files_path).unwrap();
	}
}


pub fn make_or_save_config(file: &File) {
	let paths = Paths::default();

	let uid = Uid::new();
	let uid = String::from(&uid.get().to_string());

	let main_data_config: Vec<String> = read_file(&paths.config_file_path);

	let mut success_uid = usize::MIN;

	for line in main_data_config {
		let split_line: Vec<&str> = line.split('=').collect();

		let path_after_uid = split_line.get(PATH_AFTER_EQ).unwrap().to_string();

		if path_after_uid == file.path {
			let uid_path = split_line.get(UID_BEFORE_EQ).unwrap();
			
			success_uid = uid_path.parse::<usize>().unwrap();
		}
	}

	if success_uid == usize::MIN {
		let mut path_and_uid_this_file = Path::from(uid.clone());
		path_and_uid_this_file.push('=');

		let src_path = fs::canonicalize(&file.path).unwrap().to_str().unwrap().to_string();
		
		path_and_uid_this_file.push_str(&src_path);
		path_and_uid_this_file.push('\n');
		
		append_data_in_file(&paths.config_file_path, &path_and_uid_this_file);
	}

	let mut config_file_path = Path::from(paths.config_opened_files_path);
	config_file_path.push_str(&uid);
	config_file_path.push_str(CONFIG_EXTENSION);

	let src_path = fs::canonicalize(&file.path).unwrap().to_str().unwrap().to_string();

	let mut buffer = String::from(&src_path); 
	buffer.push_str("\nscroll=");
	buffer.push_str(&file.scroll.to_string());
	// TODO: 7

	write_in_file(&config_file_path, &buffer);
}

