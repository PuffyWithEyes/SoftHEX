#![allow(deprecated)]
pub mod read;
pub mod write;
pub mod move_file;

use read::get_string_from_file;
use std::env;
use write::{IsOpen, make_or_save_config};
use move_file::move_to_opened;
use crate::etc::get_hex;


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


pub type HEX = u8;
pub type Path = String;
pub type LineNumber = u16;
type LineCounter = u16;


const CONFIG_EXTENSION: &str = ".conf";
const START_ASCII_PRINTABLE: u8 = 0x20;
const END_ASCII_PRINTABLE: u8 = 0xFF;
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
	pub text_data: String,
	pub hex_data: String,
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
		let file_data = get_string_from_file(path);
		
		let mut text = String::with_capacity(file_data.len());
		let mut hex = String::new();

		for symbol in file_data.chars() {
			if symbol as HEX > START_ASCII_PRINTABLE && symbol as HEX <= END_ASCII_PRINTABLE {
				text.push(symbol);
			} else {
				text.push('.');
			}

			let mut detail = String::with_capacity(3);
			detail.push_str(&get_hex(symbol));
			detail.push(' ');
																		
			hex.push_str(&detail);
		}
		
		let file: Self = match make_or_save_config(path, file_scroll) {
			IsOpen::Yes(path_of_conf_file) => File {
				path: Path::from(path),
				text_data: text,
				hex_data: hex.clone(),
				scroll: file_scroll,
				line_counter: hex.len() as LineCounter,
				find_text: String::new(),
				file_mode: FileState::Normal,
				conf_path: path_of_conf_file,
			},
			IsOpen::No(data_of_file) => {
				let mut new_file = File {
					path: Path::from(path),
					text_data: text,
					hex_data: hex.clone(),
					scroll: data_of_file.scroll,
					line_counter: hex.len() as LineCounter,
					find_text: String::new(),
					file_mode: FileState::Normal,
					conf_path: data_of_file.path,
				};

				move_to_opened(&mut new_file);

				new_file
			},
		};

        file
	}
}

