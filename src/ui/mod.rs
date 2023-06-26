mod draw;


use crate::{App, InputMode};
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
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('w') | KeyCode::Char('W') | KeyCode::Up => {
                        app.move_down();
                    },
                    KeyCode::Char('s') | KeyCode::Char('S') | KeyCode::Down => {
                        app.move_up();
                    },
                    KeyCode::Char('f') | KeyCode::Char('F') => {
                        app.input_mode = InputMode::FindInput;
                    },
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        return Ok(());
                    },
                    _ => {},
                },
                InputMode::FindInput => match key.code {
                    KeyCode::Enter => {
                        app.input_mode = InputMode::FindTextInput;
                        // TODO: 1
                    }
                    KeyCode::Char(c) => {
                        app.input_find.push(c);
                        thread::sleep(time::Duration::from_millis(100));
                    }
                    KeyCode::Backspace => {
                        app.input_find.pop();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
                InputMode::FindTextInput => todo!("Сделать поиск по элементам"),
                InputMode::EditingHex => todo!("Сделать рефактор hex"),
                InputMode::EditingText => todo!("Сделать рефактор текста"),
            }
        }
    }
}
