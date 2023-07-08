use std::{fs, io::{BufReader, BufRead}};
use super::Path;


pub fn read_file(path: &Path) -> Vec<String> {
    let file = fs::File::open(path).unwrap();
	let reader = BufReader::new(file);

	let mut data_vec: Vec<String> = Vec::new();

	for line in reader.lines() {
		data_vec.push(line.unwrap())
	}

	data_vec
}

