pub mod system;

use std::fs::{read_dir, remove_dir_all, ReadDir};
use std::iter::Enumerate;
use std::path::{Path,PathBuf};

use crate::arguments::cli::{CliArguments};
use crate::program::{SystemVersionDefaults};

pub(crate) fn walk_directory<P: AsRef<Path>>(location: P, valid_file: fn(&PathBuf) -> bool) -> Vec<PathBuf> {
    let mut current_directory: Vec<Enumerate<ReadDir>> = Vec::new();
    let mut valid_files: Vec<PathBuf> = Vec::new();

    if let Ok(files) = read_dir(location) {
        let mut iter = files.enumerate();

        loop {
            while let Some((_,dir_entry_result)) = iter.next() {
                if let Ok(dir_entry) = dir_entry_result {
                    let current_path = dir_entry.path();
                    if current_path.is_dir() {
                        if let Ok(files) = read_dir(current_path) {
                            current_directory.push(iter);
                            iter = files.enumerate();
                        }
                    } else if valid_file(&current_path) {
                        valid_files.push(current_path);
                    }
                    
                }
            }

            if current_directory.is_empty() {
                break;
            } else {
                iter = current_directory.pop().unwrap();
            }
        }
    }

    valid_files
}
