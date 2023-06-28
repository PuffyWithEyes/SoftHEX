mod ui;
mod files;


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
use files::File;


const LINE_NUMBER: u16 = 1_u16;
type TabIndex = usize;


pub struct App {
    opened_files: Vec<File>,
    tabs_titles: Vec<String>,
    tabs_indexes: TabIndex,
}


impl Default for App {
    fn default() -> Self {
        App {
            opened_files: Vec::new(),
            tabs_titles: Vec::new(),
            tabs_indexes: 0_usize,
        }
    }
}


fn main() -> Result<(), Box<dyn Error>> {  // TODO: 2
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::default();
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
