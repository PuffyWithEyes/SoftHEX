use std::{fs, io::Write, path};
use super::{Path, Paths, CONFIG_EXTENSION, LineCounter, read::read_file};
use chrono::Utc;


const UID_BEFORE_EQ: usize = 0_usize;
const PATH_AFTER_EQ: usize = 1_usize;
const SCROLL_AFTER_EQ: usize = 1_usize;
pub const PATH_IN_CONFIG_AT_VEC: usize = 0_usize;
pub const SCROLL_IN_CONFIG_AT_VEC: usize = 1_usize;


pub struct DataClosedFile {
	pub path: Path,
	pub scroll: LineCounter,
}


<<<<<<< HEAD
impl DataClosedFile {
	fn new(path_of_file: &Path, scroll_of_file: LineCounter) -> Self {
		DataClosedFile {
			path: path_of_file.clone(),
			scroll: scroll_of_file,
=======
impl Default for Paths {
	fn default() -> Self {
		let mut home_dir = env::home_dir().unwrap().to_str().unwrap().to_string();
		home_dir.push('/');
		
		let mut conf_path = String::from(home_dir.clone());
		conf_path.push_str(".config/");

		let mut conf_softhex_path = String::from(home_dir.clone());
		conf_softhex_path.push_str(".config/softhex/");

		let mut conf_file_path = String::from(home_dir.clone());
		conf_file_path.push_str(".config/softhex/opened_files.conf");

		let mut conf_opened_file = String::from(home_dir);
		conf_opened_file.push_str(".config/softhex/opened_files/");

		Paths {
			config_path: conf_path,
			config_softhex_path: conf_softhex_path,
			config_file_path: conf_file_path,
			config_opened_files_path: conf_opened_file,
>>>>>>> c6d433c (Fix some bugs)
		}
	}
}


pub enum IsOpen {
	Yes(Path),
	No(DataClosedFile),
}


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


fn write_data_in_conf(from_path: &Path, uid: usize, path_of_file: &Path, scroll: LineCounter) -> Path {
	let mut config_path_name_of_file = Path::from(from_path);
	config_path_name_of_file.push_str(&uid.to_string());
	config_path_name_of_file.push_str(CONFIG_EXTENSION);

	let src_path = fs::canonicalize(&path_of_file).unwrap().to_str().unwrap().to_string();

	let mut buffer = String::from(&src_path); 
	buffer.push_str("\nscroll=");
	buffer.push_str(&scroll.to_string());  // TODO: 7

	write_in_file(&config_path_name_of_file, &buffer);

	config_path_name_of_file
}


pub fn make_or_save_config(path: &Path, scroll: u16) -> IsOpen {
	let paths = Paths::default();

	let main_data_config: Vec<String> = read_file(&paths.config_file_path);

	let mut success_uid = usize::MIN;

	for line in main_data_config {
		let split_line: Vec<&str> = line.split('=').collect();

		let path_after_uid = split_line.get(PATH_AFTER_EQ).unwrap().to_string();
		
		if &path_after_uid == path {
			let uid_path = split_line.get(UID_BEFORE_EQ).unwrap();
			
			success_uid = uid_path.parse::<usize>().unwrap();
		}
	}

	if success_uid != usize::MIN {
		let closed_files = fs::read_dir(&paths.config_softhex_path).unwrap();

		let mut file_name = Path::from(success_uid.to_string());
		file_name.push_str(CONFIG_EXTENSION);

		for file in closed_files {
			if file.as_ref().unwrap().file_name().to_str().unwrap().to_string() == file_name {
				let data_closed_file = read_file(&file.unwrap().path().to_str().unwrap().to_string());

				let split_line: Vec<&str> = data_closed_file
					.get(SCROLL_IN_CONFIG_AT_VEC)
					.unwrap()
					.split('=')
					.collect();

				let scroll = split_line
					.get(SCROLL_AFTER_EQ)
					.unwrap()
					.to_string();
				let scroll = scroll.parse::<LineCounter>().unwrap();

				let path = write_data_in_conf(&paths.config_softhex_path, success_uid, path, scroll);

				let data = DataClosedFile::new(&path, scroll);
				
				return IsOpen::No(data);
			}
		}

		return IsOpen::Yes(
			write_data_in_conf(&paths.config_opened_files_path, success_uid, path, scroll)
		);
	} else {
		let datetime_for_uid = Utc::now();
		let uid = datetime_for_uid.timestamp() as usize;
		
		let mut path_and_uid_this_file = Path::from(uid.to_string());
		path_and_uid_this_file.push('=');

		let src_path = fs::canonicalize(&path).unwrap().to_str().unwrap().to_string();
		
		path_and_uid_this_file.push_str(&src_path);
		path_and_uid_this_file.push('\n');
		
		append_data_in_file(&paths.config_file_path, &path_and_uid_this_file);
		
		return IsOpen::Yes(
			write_data_in_conf(&paths.config_opened_files_path, uid, path, scroll)
		);
	}
}

