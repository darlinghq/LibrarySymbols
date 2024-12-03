use crate::helper::read_dir_ignore_errs;

use std::vec::Vec;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct CommonSymbolLocations {
    start_path: PathBuf,
    valid_paths: Vec<PathBuf>
}

impl CommonSymbolLocations {
    pub fn new(path: &Path) -> CommonSymbolLocations {
        let mut common_system = CommonSymbolLocations {
            start_path: path.to_path_buf(),
            valid_paths: Vec::new(),
        };
        
        common_system.calculate_and_add_paths(path);

        common_system
    }

    fn calculate_and_add_paths(&mut self, path: &Path) {
        let mut explorer = SystemFolderExplorer::new(path);
        
        while !explorer.is_empty() {
            if let Some((current_path, system_path)) = explorer.grab_folders() {
                self.add_framework_and_library_paths(&current_path, &system_path);
            }
        }
    }

    fn add_framework_and_library_paths(&mut self, root_path: &Path, system_folder: &Path) {
        let library_framework = system_folder.join("Library/Frameworks");
        if library_framework.is_dir() {
            println!("Adding '{}' to search list", library_framework.to_string_lossy());
            self.valid_paths.push(library_framework);
        }

        let library_privateframework = system_folder.join("Library/PrivateFrameworks");
        if library_privateframework.is_dir() {
            println!("Adding '{}' to search list", library_privateframework.to_string_lossy());
            self.valid_paths.push(library_privateframework);
        }

        let usr_lib = root_path.join("usr/lib");
        if usr_lib.is_dir() {
            println!("Adding '{}' to search list", usr_lib.to_string_lossy());
            self.valid_paths.push(usr_lib);
        }
    }

    pub fn get_starting_path(&self) -> &Path {
        &self.start_path
    }

    pub fn get_paths(&self) -> &Vec<PathBuf> {
        &self.valid_paths
    }
}


const SYSTEM_FOLDER_BLACKLIST: [&str; 5] = ["Cryptexes","Library","Developer","Applications","Volumes"];

#[derive(Debug)]
pub struct SystemFolderExplorer {
    pub paths_to_explore: Vec<PathBuf>
}

impl SystemFolderExplorer {
    pub fn new(first_path: &Path) -> SystemFolderExplorer {
        let mut paths_to_explore = Vec::new();

        paths_to_explore.push(first_path.to_path_buf());

        SystemFolderExplorer {
            paths_to_explore
        }
    }

    pub fn is_empty(&self) -> bool {
        self.paths_to_explore.is_empty()
    }

    /// With modern versions of macOS, there are multiple locations that have either
    /// 'System/Library/Frameworks', 'System/Library/PrivateFrameworks', or 'usr/lib':
    /// 
    /// * '/'
    /// * '/System/DriverKit/'
    /// * '/System/iOSSupport/'
    fn inspect_and_add_system_subfolder(&mut self, system_folder: &Path) {
        for file_in_system in read_dir_ignore_errs(&system_folder) {
            let file_to_check = file_in_system.path();

            if !file_to_check.is_dir() || Self::is_folder_blacklisted(&file_to_check) {
                continue;
            }

            self.paths_to_explore.push(file_to_check);
        }
    }

    /// The first PathBuf is the current path. The second PathBuf is the "System" folder appended 
    /// to the current path.
    /// 
    /// Note: the second PathBuf is not guaranteed to exist.
    pub fn grab_folders(&mut self) -> Option<(PathBuf,PathBuf)> {
        let Some(current_folder) = self.paths_to_explore.pop() else { return None };
        let system_folder = current_folder.join("System");

        self.inspect_and_add_system_subfolder(&system_folder);
        Some( (current_folder, system_folder) )
    }

    fn is_folder_blacklisted(file_to_check: &Path) -> bool {
        for blacklist_folder_name in SYSTEM_FOLDER_BLACKLIST {
            if file_to_check.ends_with(blacklist_folder_name) {
                return true;
            }
        }

        false
    }
}
