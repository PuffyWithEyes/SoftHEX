mod draw;


use crate::App;
use draw::ui;
use crossterm::event::{self, Event, KeyCode};
use tui::{
    backend::Backend,
    Terminal,
};
use std::{thread, time, io};


pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match app.app_mode {
                AppState::Normal => match key.code {
                    KeyCode::Char('w') | KeyCode::Char('W') | KeyCode::Up => {
                        let mut file = app.opened_files.get(0).unwrap();
                        file.page_down();
                    },
                    KeyCode::Char('s') | KeyCode::Char('S') | KeyCode::Down => {
                        let mut file = app.opened_files.get(0).unwrap();
                        file.page_up();
                    },
                    KeyCode::Char('f') | KeyCode::Char('F') => {
                        app.app_mode = AppState::FindInput;
                    },
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        return Ok(());
                    },
                    _ => {},
                },
                AppState::FindInput => match key.code {
                    KeyCode::Enter => {
                        app.app_mode = AppState::FindTextInput;
                        // TODO: 1
                    }
                    KeyCode::Char(c) => {
                        let mut find_text = app.
                        app.input_find.push(c);
                        thread::sleep(time::Duration::from_millis(100));
                    }
                    KeyCode::Backspace => {
                        app.input_find.pop();
                    }
                    KeyCode::Esc => {
                        app.app_mode = AppState::Normal;
                    }
                    _ => {}
                },
                AppState::FindTextInput => todo!("Сделать поиск по элементам"),
                AppState::EditingHex => todo!("Сделать рефактор hex"),
                AppState::EditingText => todo!("Сделать рефактор текста"),
            }
        }
    }
}
