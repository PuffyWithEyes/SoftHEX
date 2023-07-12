mod ui;
mod files;

use ui::run_app;
use crossterm::{
    event::EnableMouseCapture,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io, env, fs, path};
use tui::{
    backend::CrosstermBackend,
    Terminal,
};
use files::{File, Path, write::{Paths, make_config_file_if_not_exist}, read::read_file, LineNumber};


type TabIndex = usize;


pub struct App {
    opened_files: Vec<File>,
    tabs_titles: Vec<String>,
    tabs_indexes: TabIndex,
}


impl App {
	pub fn new() -> Self {
		App {
			opened_files: Vec::new(),
			tabs_titles: Vec::new(),
			tabs_indexes: usize::MIN,
		}
	}

	pub fn add_file(&mut self, path: &Path) {
		let file = File::new(path);
		
		let file_name = path::Path::new(path);
		let file_name = file_name.file_name().unwrap().to_str().unwrap();
		
		self.opened_files.insert(0, file);
		self.tabs_titles.insert(0, file_name.to_string());
	}

	pub fn add_complete_file(&mut self, file: &File) {
		let file_name = path::Path::new(&file.path);
		let file_name = file_name.file_name().unwrap().to_str().unwrap();
		
		self.opened_files.insert(0, file.clone());
		self.tabs_titles.insert(0, file_name.to_string());
	}

	pub fn next_tab(&mut self) {
		self.tabs_indexes = (self.tabs_indexes + 1) % self.opened_files.len();
	}

	pub fn previous_tab(&mut self) {
		if self.tabs_indexes > 0 {
			self.tabs_indexes -= 1;
		} else {
			self.tabs_indexes = self.opened_files.len() - 1;
		}
	}
}


fn load_opened_files_in_app_buffer(app: &mut App) {
	let default_paths = Paths::default();

	make_config_file_if_not_exist(&default_paths);
	
	let paths = fs::read_dir(&default_paths.config_opened_files_path).unwrap();

	for path in paths {
		let config_file_data = read_file(&path.unwrap().path().to_str().unwrap().to_string());

		let path_to_file = config_file_data.get(0).unwrap();
		let scroll_of_file = config_file_data.get(1).unwrap()
			.split('=')
			.collect::<Vec::<&str>>()
			.get(1)
			.unwrap()
			.parse::<LineNumber>()
			.unwrap();

		let file = File::new_from_config(&path_to_file, &scroll_of_file);

		app.add_complete_file(&file);
	}
}


#[cfg(target_os = "linux")]
fn main() -> Result<(), Box<dyn Error>> {
	let args: Vec<String> = env::args().collect();

	let mut app = App::new();

	let mut is_zero_iter = true;

	load_opened_files_in_app_buffer(&mut app);
	
	for arg in args {
		if is_zero_iter {
			is_zero_iter = false;
			continue;
		}
		
		if arg.to_lowercase() == "--help" || arg.to_lowercase() == "-h" {
			println!("So far there is nothing here :)");

			return Ok(());
		} else {
			let file_path = path::Path::new(&arg);

			if file_path.exists() && file_path.is_file() {
				app.add_file(&arg);
			} else {
				panic!("This directory or yhis file doesen't exist ({})", arg);
			}
		}
	}
	
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        EnableMouseCapture,  // Edit in disable
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

