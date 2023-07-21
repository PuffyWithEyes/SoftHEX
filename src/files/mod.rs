#![allow(deprecated)]

pub mod read;
pub mod write;
pub mod move_file;

use read::read_file;
use std::env;
use write::{IsOpen, make_or_save_config};
use move_file::move_to_opened;


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
		conf_file_path.push_str(".config/softhex/opened_files.conf");

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


pub type Path = String;
pub type LineNumber = u16;
type LineCounter = u16;


const LINE_NUMBER: u16 = 1_u16;
const START_LINE: u16 = u16::MIN;


#[derive(Clone)]
pub enum FileState {
    Normal,
    FindTextInput,
	FindText,
    EditingHex,
    EditingText,
	Saved,
}


#[derive(Clone)]
pub struct File {
    pub path: Path,
    pub data: Vec<String>,
    pub scroll: LineNumber,
    line_counter: LineCounter,
    pub find_text: String,
    pub file_mode: FileState,
	conf_path: Path,
}


impl File {
    pub fn new(path: &Path) -> Self {
		Self::new_object(path, START_LINE)
    }

	pub fn new_from_config(path: &Path, scroll_file: LineNumber) -> Self {
		Self::new_object(path, scroll_file)
	}
 
    pub fn page_up(&mut self) {
        if self.scroll != self.line_counter {
            self.scroll += LINE_NUMBER;
        }
    }

    pub fn page_down(&mut self) {
        if self.scroll != 0 {
            self.scroll -= LINE_NUMBER;
        }
    }

	fn new_object(path: &Path, file_scroll: LineNumber) -> Self {
		let mut file: Self = match make_or_save_config(path, file_scroll) {
			IsOpen::Yes(path_of_conf_file) => File {
				path: Path::from(path),
				data: read_file(path),
				scroll: file_scroll,
				line_counter: u16::MIN,
				find_text: String::new(),
				file_mode: FileState::Normal,
				conf_path: path_of_conf_file,
			},
			IsOpen::No(path_of_conf_file) => {
				let mut new_file = File {
					path: Path::from(path),
					data: read_file(path),
					scroll: file_scroll,
					line_counter: u16::MIN,
					find_text: String::new(),
					file_mode: FileState::Normal,
					conf_path: path_of_conf_file,
				};

				move_to_opened(&mut new_file);

				new_file
			},
		};
		
        file.line_counter = file.data.len() as LineCounter;

        file
	}
}

