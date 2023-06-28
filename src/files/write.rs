use std::fs;
use std::io::Write;
use super::{Path, File};


pub fn make_or_save_config(path: &Path, file: &File) {

}


pub fn write_in_file(path: &Path, data: &String) {
    let mut file = fs::File::create("output.txt")?;
    file.write_all(b"Hello, world!")?;
    Ok(())
}
