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
use files::{File, Path, write::CONFIG_OPENED_FILES_PATH, read::read_file};


const LINE_NUMBER: u16 = 1_u16;
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
			tabs_indexes: 0_usize,
		}
	}

	pub fn add_file(&mut self, path: &Path) {
		let file = File::new(path);
		
		let file_name = path::Path::new(path);
		let file_name = file_name.file_name().unwrap().to_str().unwrap();
		
		self.opened_files.insert(0, file);
		self.tabs_titles.insert(0, file_name.to_string());
	}
}


fn load_all_files_in_app_buffer() {
	let paths = fs::read_dir(CONFIG_OPENED_FILES_PATH).unwrap();

	for file in paths {
		let data = read_file(&file.unwrap().path().to_str().unwrap().to_string()).unwrap();
		todo!("Написать пробежку по файлу конфига и последующую загрузку его в буффер");
	}
}



fn main() -> Result<(), Box<dyn Error>> {  // TODO: 2
	let args: Vec<String> = env::args().collect();

	let app = App::new();

	for arg in args {
		if arg.to_lowercase() == "--help" || arg.to_lowercase() == "-h" {
			todo!("Сделать help")
		} else {
			let file_path = fs::metadata(arg);

			if file_path.is_ok() {
				
			} else {
				
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

