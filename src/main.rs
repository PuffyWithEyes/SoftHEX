mod ui;
mod read_files;


use ui::run_app;
use crossterm::{
    event::EnableMouseCapture,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::CrosstermBackend,
    Terminal,
};


const LINE_NUMBER: u16 = 1_u16;
const CORRECT_OUTPUT: u16 = 10_u16;


pub enum InputMode {
    Normal,
    FindInput,
    FindTextInput,
    EditingHex,
    EditingText,
}


pub struct App {
    input_find: String,
    input_mode: InputMode,
    hex: String,
    text: String,
    scroll: u16,
}


impl App {
    pub fn new() -> Self {
        App {
            input_find: String::new(),
            input_mode: InputMode::Normal,
            hex: String::new(),
            text: String::new(),
            scroll: 0,
        }
    }

    pub fn move_down(&mut self) {
        self.scroll += LINE_NUMBER;
        self.scroll %= CORRECT_OUTPUT;
    }

    pub fn move_up(&mut self) {
        if self.scroll != 0 {
            self.scroll -= LINE_NUMBER;
            self.scroll %= CORRECT_OUTPUT;
        }
    }
}


fn main() -> Result<(), Box<dyn Error>> {  // TODO: 2
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new();
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
