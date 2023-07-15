mod draw;

use crate::{
	App,
	files::{
		FileState, Paths,
		write::{PATH_IN_CONFIG_AT_VEC, make_or_save_config},
		move_file::move_file,
		read::read_file,
	},
};
use draw::ui;
use crossterm::event::{self, Event, KeyCode};
use tui::{
    backend::Backend,
    Terminal,
};
use std::{io, fs};


pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
			match app.get_current_file().file_mode {
				FileState::Normal => {
					match key.code {
						KeyCode::Char('d') | KeyCode::Char('D') | KeyCode::Char('в') | KeyCode::Char('В') |
						KeyCode::Right => {
							app.next_tab();
						},
						KeyCode::Char('a') | KeyCode::Char('A') | KeyCode::Char('ф') | KeyCode::Char('Ф') |
						KeyCode::Left => {
							app.previous_tab();
						},
						KeyCode::Char('w') | KeyCode::Char('W') | KeyCode::Char('ц') | KeyCode::Char('Ц') |
						KeyCode::Up => {
							let file = app.get_current_file_mut();

							file.page_up();
						},
						KeyCode::Char('s') | KeyCode::Char('S') | KeyCode::Char('ы') | KeyCode::Char('Ы') |
						KeyCode::Down => {
							let file = app.get_current_file_mut();

							file.page_down();
						},
						KeyCode::Char('t') | KeyCode::Char('T') | KeyCode::Char('е') | KeyCode::Char('Е') => {
							let file = app.get_current_file_mut();

							file.file_mode = FileState::EditingText;
						},
						KeyCode::Char('h') | KeyCode::Char('H') | KeyCode::Char('р') | KeyCode::Char('Р') => {
							let file = app.get_current_file_mut();

							file.file_mode = FileState::EditingHex;
						},
						KeyCode::F(5) => {  
							let file = app.get_current_file_mut();
							
							make_or_save_config(&file);

							file.file_mode = FileState::Saved;
						},
						KeyCode::Char('c') | KeyCode::Char('C') | KeyCode::Char('с') | KeyCode::Char('С') => {
							if app.opened_files.len() == 1 {  // TODO: 11
								return Ok(());
							} else {
								let default_paths = Paths::default();
								let opened_files = fs::read_dir(default_paths.config_opened_files_path).unwrap();

								for file_path in opened_files {
									let config_file_data = read_file(
										&file_path.unwrap().path().to_str().unwrap().to_string()
									);

									let path_of_file_in_conf = config_file_data.get(PATH_IN_CONFIG_AT_VEC);

									//if path_of_file_in_conf == 
								}
								
								//move_file(from, to);
								
								app.close_current_tab();
							}
						},
						KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Char('й') | KeyCode::Char('Й') => {
							return Ok(());  // TODO: 11
						},
						_ => {},
					}
				},
				FileState::FindTextInput => {
					todo!("Сделать ввод текста для поиска")
				},
				FileState::FindText => {
					todo!("Сделать сам поиск")
				},
				FileState::EditingHex => {
					todo!("Сделать изменение hex текста")
				},
				FileState::EditingText => {
					todo!("Сделать изменение обычного текста")
				},
				FileState::Saved => {
					todo!("Сделать уведомление о том, что успешно было сохранено")
				},
			}
        }
    }
}

