use std::{fs::DirEntry, path::Path};


pub fn read_dir_ignore_errs(path: &Path) -> Vec<DirEntry> {
    let mut clean_list = Vec::new();

    if let Ok(files) = std::fs::read_dir(path) {
        for result_file in files {
            if let Ok(file) = result_file {
                clean_list.push(file);
            }
        }
    }

    clean_list
}
