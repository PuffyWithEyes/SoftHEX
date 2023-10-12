mod ui;
mod files;
mod etc;

use ui::run_app;
use crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io, env, fs, path};
use tui::{
    backend::CrosstermBackend,
    Terminal,
};
use files::{
	File, Path, LineNumber, Paths,
	write::{PATH_IN_CONFIG_AT_VEC, SCROLL_IN_CONFIG_AT_VEC, make_config_file_if_not_exist, make_or_save_config},
	read::{read_file, number_of_opened_files},
	move_file::move_to_closed,
};


type TabIndex = usize;


pub struct App {
    opened_files: Vec<File>,
    tabs_titles: Vec<String>,
    current_index: TabIndex,
	open_file_text: Path,
}


impl App {
	pub fn new() -> Self {
		App {
			opened_files: Vec::new(),
			tabs_titles: Vec::new(),
			current_index: usize::MIN,
			open_file_text: Path::new(),
		}
	}

	pub fn add_file(&mut self, path: &Path) {  // TODO: 12
		let file = File::new(path);
		
		let file_name = path::Path::new(path);
		let file_name = file_name.file_name().unwrap().to_str().unwrap();
		
		self.opened_files.insert(0, file);
		self.tabs_titles.insert(0, file_name.to_string());
	}

	pub fn add_complete_file(&mut self, file: &File) {  // TODO: 12
		let file_name = path::Path::new(&file.path);
		let file_name = file_name.file_name().unwrap().to_str().unwrap();
		
		self.opened_files.insert(0, file.clone());
		self.tabs_titles.insert(0, file_name.to_string());
	}

	pub fn next_tab(&mut self) {
		self.current_index = (self.current_index + 1) % self.opened_files.len();
	}

	pub fn previous_tab(&mut self) {
		if self.current_index > 0 {
			self.current_index -= 1;
		} else {
			self.current_index = self.opened_files.len() - 1;
		}
	}

	pub fn close_current_tab(&mut self) {
		let current_index = self.current_index;

		make_or_save_config(&self.opened_files[current_index].path, self.opened_files[current_index].scroll);
		move_to_closed(&self.opened_files[current_index]);

		self.opened_files.remove(current_index);
		self.tabs_titles.remove(current_index);

		if current_index != 0 {
			self.current_index -= 1;
		}
	}

	pub fn get_current_path_of_file(&self) -> Path {
		let file = self.get_current_file();
		
		file.path.clone()
	}

	pub fn get_current_file(&self) -> File {
		let index_opened_tab = self.current_index;
		
		self.opened_files[index_opened_tab].clone()
	}

	pub fn get_current_file_mut(&mut self) -> &mut File {
		&mut self.opened_files[self.current_index]
	}

	pub fn open_file_wth_ui(&mut self) {
		let file = self.get_current_file_mut();
		file.file_mode = crate::files::FileState::Normal;  // TODO: Потом возможно сделать обработку ошибок
		
		let mut path = Path::from(self.open_file_text.clone());
		path = fs::canonicalize(path).unwrap().to_str().unwrap().to_string();
		
		if self.is_file_open_in_tabs(&path) {
			return;
		}
		
		self.add_file(&path);
	}

	pub fn is_file_open_in_tabs(&self, path: &Path) -> bool {
		for file in &self.opened_files {
			if &file.path == path {
				return true
			}
		}

		false
	}
}


fn load_opened_files_in_app_buffer(app: &mut App) {
	let default_paths = Paths::default();

	make_config_file_if_not_exist(&default_paths);
	
	let paths = fs::read_dir(&default_paths.config_opened_files_path).unwrap();
	
	for path in paths {
		let config_file_data = read_file(&path.unwrap().path().to_str().unwrap().to_string());

		let path_to_file = config_file_data.get(PATH_IN_CONFIG_AT_VEC).unwrap();
		let scroll_of_file = config_file_data.get(SCROLL_IN_CONFIG_AT_VEC).unwrap() 
			.split('=')
			.collect::<Vec::<&str>>()
			.get(1)
			.unwrap()
			.parse::<LineNumber>()
			.unwrap();

		let file = File::new_from_config(&path_to_file, scroll_of_file);

		app.add_complete_file(&file);
	}
}


fn print_help() {
	println!(
		"Usage: softhex [FILES TO OPEN]
   or: softhex [-h or --help]

Terminal HEX Viewier"	
	);
}


#[cfg(target_os = "linux")]
fn main() -> Result<(), Box<dyn Error>> {
	let args: Vec<String> = env::args().collect();

	let mut app = App::new();

	load_opened_files_in_app_buffer(&mut app);

	if args.len() == 1 && number_of_opened_files() == 0 {
		print_help();
		
		return Ok(());
	} else {  // TODO: 12
		let mut is_zero_iter = true;
		
		for arg in args {
			if is_zero_iter {
				is_zero_iter = false;
				continue;
			}
			
			if arg.to_lowercase() == "--help" || arg.to_lowercase() == "-h" {
				print_help();

				return Ok(());
			} else {
				let file_path = path::Path::new(&arg);

				if file_path.exists() && file_path.is_file() {
					let file_path: Path = fs::canonicalize(&arg)?.to_str().unwrap().to_string();

					if app.is_file_open_in_tabs(&file_path) {
						continue;
					} 
					
					app.add_file(&file_path);
				} else {
					panic!("This directory or this file doesen't exist ({})", arg);
				}
			}
		}
	}
	
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, DisableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

