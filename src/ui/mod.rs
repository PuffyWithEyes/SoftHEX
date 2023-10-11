mod draw;

use crate::{
	App,
	files::{
		FileState,
		write::make_or_save_config,
	},
};
use draw::ui;
use crossterm::event::{self, Event, KeyCode};
use tui::{
    backend::Backend,
    Terminal,
};
use std::io;


pub fn run_app<B>(
    terminal: &mut Terminal<B>,
    mut app: App,
) -> io::Result<()>
where B: Backend {
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
							
							file.page_down();
						},
						KeyCode::Char('s') | KeyCode::Char('S') | KeyCode::Char('ы') | KeyCode::Char('Ы') |
						KeyCode::Down => {
							let file = app.get_current_file_mut();

							file.page_up();
						},
						KeyCode::Char('h') | KeyCode::Char('H') | KeyCode::Char('р') | KeyCode::Char('Р') => {
							let file = app.get_current_file_mut();

							file.file_mode = FileState::EditingHex;
						},
						KeyCode::Char('f') | KeyCode::Char('F') | KeyCode::Char('а') | KeyCode::Char('А') => {
							let file = app.get_current_file_mut();

							file.file_mode = FileState::FindTextInput;
						},
						KeyCode::F(5) => {  
							let file = app.get_current_file_mut();

							make_or_save_config(&file.path, file.scroll);

							file.file_mode = FileState::Saved;
						},
						KeyCode::Char('c') | KeyCode::Char('C') | KeyCode::Char('с') | KeyCode::Char('С') => {
							app.close_current_tab();
							
							if app.opened_files.len() == 0 {								
								return Ok(());
							}
						},
						KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Char('й') | KeyCode::Char('Й') => {
							for file in app.opened_files {
								make_or_save_config(&file.path, file.scroll);
							}
							
							return Ok(());  // TODO: 11
						},
						KeyCode::Char('o') | KeyCode::Char('O') | KeyCode::Char('щ') | KeyCode::Char('Щ') => {
							let file = app.get_current_file_mut();

							file.file_mode = FileState::OpenFile;
						},
						_ => {},
					}
				},
				FileState::FindTextInput => {
					match key.code {
						KeyCode::Esc => {
							let file = app.get_current_file_mut();
							file.file_mode = FileState::Normal;
						},
						KeyCode::Enter => {
							let file = app.get_current_file_mut();
							file.file_mode = FileState::FindTextInput;
						},
						KeyCode::Backspace => {
							let file = app.get_current_file_mut();
							file.find_text.pop();
						}
						KeyCode::Char(symbol) => {
							let file = app.get_current_file_mut();
							file.find_text.push(symbol);
						},
						KeyCode::Left => {
							app.previous_tab();
						},
						KeyCode::Right => {
							app.next_tab();
						},
						_ => {},
					}
				},
				FileState::FindText => {
					todo!("Сделать сам поиск")
				},
				FileState::EditingHex => {
					todo!("Сделать изменение hex текста")
				},
				FileState::Saved => {
					todo!("Сделать уведомление о том, что успешно было сохранено")
				},
				FileState::OpenFile => {
					match key.code {
						KeyCode::Esc => {
							let file = app.get_current_file_mut();
							file.file_mode = FileState::Normal;
						},
						KeyCode::Enter => {
							app.open_file_wth_ui();
						},
						KeyCode::Backspace => {
							app.open_file_text.pop();
						},
						KeyCode::Char(symbol) => {
							app.open_file_text.push(symbol);
						},
						KeyCode::Left => {
							app.previous_tab();
						},
						KeyCode::Right => {
							app.next_tab();
						},
						_ => {},
					}
				},
			}
        }
    }
}

