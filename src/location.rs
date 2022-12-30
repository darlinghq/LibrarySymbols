pub mod system;

use std::fs::{read_dir, ReadDir};
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


#[derive(Debug)]
pub struct ResultsLocation {
    pub os_version_path: PathBuf,
    pub temp_path: PathBuf,
}

impl ResultsLocation {
    pub fn new(arguments: &CliArguments, system_version: SystemVersionDefaults) -> ResultsLocation {
        const TEMP_DIR: &str = "tmp";

        let version_folder = format!("{} ({})", system_version.product_version, system_version.product_build_version);
        let system_version = &system_version.product_name;
        
        let os_version_path = arguments.results_path.as_path().join(system_version).join(version_folder);
        let temp_path =  os_version_path.join(TEMP_DIR);

        Self {
            os_version_path,
            temp_path,
        }
    }
}