pub mod read;
pub mod write;
pub mod move_file;

use read::read_file;


pub type Path = String;
type LineNumber = u16;
type LineCounter = u16;


const LINE_NUMBER: u16 = 1_u16;
const START_LINE: u16 = 0_u16;


pub enum FileState {
    Normal,
    FindInput,
    FindTextInput,
    EditingHex,
    EditingText,
}


pub struct File {
    path: Path,
    pub data: Vec<String>,
    pub scroll: LineNumber,
    line_counter: LineCounter,
    find_text: String,
    file_mode: FileState,
}


impl File {
    pub fn new(path: &Path) -> Self {
        let mut file = File {
            path: Path::from(path),
            data: read_file(path),
            scroll: START_LINE,
            line_counter: 0_u16,
            find_text: String::new(),
			file_mode: FileState::Normal,
        };

        file.line_counter = file.data.len() as LineCounter;

        file
    }

	pub fn new_from_config() -> Self {
		todo!("Сделать создание объекта из конфига")
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

