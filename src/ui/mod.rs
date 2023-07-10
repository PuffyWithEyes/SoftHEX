mod draw;

use crate::{App, files::{FileState, File}};
use draw::ui;
use crossterm::event::{self, Event, KeyCode};
use tui::{
    backend::Backend,
    Terminal,
};
use std::io;


pub fn get_file_from_vec(app: &mut App) -> File {
	let index_opened_tab = app.tabs_indexes;

	app.opened_files.get(index_opened_tab).unwrap().clone()
}


pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
) -> io::Result<()> {
	
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
			match get_file_from_vec(&mut app).file_mode {
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
							let mut file = get_file_from_vec(&mut app);

							file.page_up();
						},
						KeyCode::Char('s') | KeyCode::Char('S') | KeyCode::Char('ы') | KeyCode::Char('Ы') |
						KeyCode::Down => {
							let mut file = get_file_from_vec(&mut app);

							file.page_down();
						},
						KeyCode::Char('t') | KeyCode::Char('T') | KeyCode::Char('е') | KeyCode::Char('Е') => {
							let mut file = get_file_from_vec(&mut app);

							file.file_mode = FileState::EditingText;
						},
						KeyCode::Char('h') | KeyCode::Char('H') | KeyCode::Char('р') | KeyCode::Char('Р') => {
							let mut file = get_file_from_vec(&mut app);

							file.file_mode = FileState::EditingHex;
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
			}
        }
    }
}

