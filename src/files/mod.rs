pub mod read;
pub mod write;
pub mod move_file;

use read::read_file;


pub type Path = String;
pub type LineNumber = u16;
type LineCounter = u16;


const LINE_NUMBER: u16 = 1_u16;
const START_LINE: u16 = u16::MIN;


#[derive(Clone)]
pub enum FileState {
    Normal,
    FindTextInput,
	FindText,
    EditingHex,
    EditingText,
}


#[derive(Clone)]
pub struct File {
    pub path: Path,
    pub data: Vec<String>,
    pub scroll: LineNumber,
    line_counter: LineCounter,
    pub find_text: String,
    pub file_mode: FileState,
}


impl File {
    pub fn new(path: &Path) -> Self {
        let mut file = File {
            path: Path::from(path),
            data: read_file(path),
            scroll: START_LINE,
            line_counter: u16::MIN,
            find_text: String::new(),
			file_mode: FileState::Normal,
        };

        file.line_counter = file.data.len() as LineCounter;

        file
    }

	pub fn new_from_config(path: &Path, scroll_file: &LineNumber) -> Self {
		File {
			path: Path::from(path),
			data: read_file(path),
			scroll: *scroll_file,
			line_counter: u16::MIN,
			find_text: String::new(),
			file_mode: FileState::Normal,
		}
	}
 
    pub fn page_up(&mut self) {
        if self.scroll != self.line_counter {
            self.scroll += LINE_NUMBER;
        }
    }

    pub fn page_down(&mut self) {
        if self.scroll != 0 {
            self.scroll -= LINE_NUMBER;
        }
    }
}

