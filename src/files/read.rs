use std::{fs, io::{BufReader, BufRead, Read}};
use super::{Path, Paths};


pub fn number_of_opened_files() -> usize {
	let paths = Paths::default();
	let mut counter = 0_usize;
	
	let dirs = fs::read_dir(paths.config_opened_files_path).unwrap();

	for _ in dirs {
		counter += 1;
	}

	counter
}


pub fn read_file(path: &Path) -> Vec<String> {
    let file = fs::File::open(path).unwrap();
	let reader = BufReader::new(file);

	let mut data_vec: Vec<String> = Vec::new();

	for line in reader.lines() {
		data_vec.push(line.unwrap())
	}

	data_vec
}


pub fn get_string_from_file(path: &Path) -> String {
	let mut file = fs::File::open(path).unwrap();
	let mut data = String::new();

	file.read_to_string(&mut data).unwrap();

	data
}

