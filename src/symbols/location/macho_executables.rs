
use crate::helper::read_dir_ignore_errs;
use crate::symbols::macho::is_file_macho;

use std::path::{Path, PathBuf};

pub struct MachoExecutableLocation {
    macho_paths: Vec<PathBuf>,
}

impl MachoExecutableLocation {
    pub fn new(path: &Path) -> MachoExecutableLocation {
        let mut macho_library_location = MachoExecutableLocation {
            macho_paths: Vec::new(),
        };

        macho_library_location.locate_macho_location(path);

        macho_library_location
    }

    fn locate_macho_location(&mut self, path: &Path) {
        let mut search_list = Vec::new();
        search_list.push(path.to_path_buf());

        while let Some(path_to_inspect) = search_list.pop() {
            for current_path in read_dir_ignore_errs(&path_to_inspect) {
                let subpath= current_path.path();
                
                if subpath.is_symlink() {
                    // Let's ignore symlinks (otherwise we will deal with duplicate entries...)
                    println!("Ignoring symlink {}", subpath.to_string_lossy());
                    continue;
                } else if subpath.is_file() && is_file_macho(&subpath) {
                    self.macho_paths.push(subpath);
                } else if subpath.is_dir() {
                    search_list.push(subpath);
                } 
            }
        }
    }

    pub fn get_paths(&self) -> &Vec<PathBuf> {
        &self.macho_paths
    }
}