use std::{fs, env, io::Write, path};
use super::{Path, File, read::read_file};
use uid::Id as IdT;


pub struct Paths {
	pub config_path: String,
	pub config_softhex_path: String,
	pub config_file_path: String,
	pub config_opened_files_path: String,
}


impl Default for Paths {
	fn default() -> Self {
		let mut home_dir = env::home_dir().unwrap().to_str().unwrap().to_string();
		home_dir.push('/');
		
		let mut conf_path = String::from(home_dir.clone());
		conf_path.push_str(".config/");

		let mut conf_softhex_path = String::from(home_dir.clone());
		conf_softhex_path.push_str(".config/softhex/");

		let mut conf_file_path = String::from(home_dir.clone());
		conf_file_path.push_str(".config/softhex/opened_files.conf/");

		let mut conf_opened_file = String::from(home_dir);
		conf_opened_file.push_str(".config/softhex/opened_files/");

		Paths {
			config_path: conf_path,
			config_softhex_path: conf_softhex_path,
			config_file_path: conf_file_path,
			config_opened_files_path: conf_opened_file,
		}
	}
}


const CONFIG_EXTENSION: &str = ".conf";


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
		write_in_file(&paths.config_file_path, &"".to_string());
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

		let path_after_uid = split_line.get(1).unwrap().to_string();

		if path_after_uid == file.path {
			let uid_path = split_line.get(0).unwrap();
			
			success_uid = uid_path.parse::<usize>().unwrap();
		}
	}

	if success_uid == usize::MIN {
		let mut path_and_uid_this_file = Path::from(uid.clone());
		path_and_uid_this_file.push('=');
		path_and_uid_this_file.push_str(&file.path);

		append_data_in_file(&paths.config_file_path, &path_and_uid_this_file);
	}

	let mut config_file_path = Path::from(paths.config_opened_files_path);
	config_file_path.push_str(&uid);
	config_file_path.push_str(CONFIG_EXTENSION);

	let mut buffer = String::from(&file.path);
	buffer.push('\n'); 
	
	buffer.push_str("\nscroll=");
	buffer.push_str(&file.scroll.to_string());
	// TODO: 7

	append_data_in_file(&config_file_path, &buffer);

	write_in_file(&config_file_path, &buffer);
}

