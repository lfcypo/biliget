use std::path::{Path, PathBuf};
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref TEMP_FILE_LIST: Mutex<Vec<PathBuf>> = Mutex::new(Vec::new());
}

pub fn add_temp_file(file: &Path) {
    let mut temp_file_list = TEMP_FILE_LIST.lock().unwrap();
    temp_file_list.push(file.to_path_buf());
}

pub fn drop_temp_file() {
    let mut temp_file_list = TEMP_FILE_LIST.lock().unwrap();
    for file in temp_file_list.iter() {
        std::fs::remove_file(file).ok();
    }
    temp_file_list.clear();
}
