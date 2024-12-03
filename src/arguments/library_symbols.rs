use std::{fs::{create_dir_all, remove_dir_all}, path::{Path, PathBuf}};

use crate::program::SystemVersionDefaults;

use super::cli::CliArguments;

pub struct LibrarySymbolsArguments {
    root_path: PathBuf,
    cryptexes_os_path: Option<PathBuf>,
    os_symbol_path: PathBuf,
    temp_path: PathBuf,
}

impl LibrarySymbolsArguments {
    pub fn new(arguments: CliArguments) -> LibrarySymbolsArguments {
        let root_path = arguments.base_path;
        let cryptexes_os_path = arguments.cryptexes_os_path;
        
        let output_path = arguments.root_path;
        let os_symbol_path = Self::calculate_version_path(&root_path, &output_path);
        let temp_path = Self::calculate_temp_folder(&os_symbol_path);

        LibrarySymbolsArguments {
            root_path,
            cryptexes_os_path,
            os_symbol_path,
            temp_path,
        }
    }

    //
    // Getters
    //

    pub fn get_root_path(&self) -> &Path {
        &self.root_path
    }

    pub fn get_cryptexes_os_path(&self) -> Option<&PathBuf> {
        self.cryptexes_os_path.as_ref()
    }

    pub fn get_os_symbol_path(&self) -> &Path {
        &self.os_symbol_path
    }

    pub fn get_temp_path(&self) -> &Path {
        &self.temp_path
    }

    //
    // Calcuate Methods
    //

    fn calculate_version_path(root_path: &Path, output_path: &Path) -> PathBuf {
        let system_version = SystemVersionDefaults::new(root_path);
        
        // ex: os_symbol_folder would become 'macOS/15.1.1 (24B91)'
        let os_symbol_folder = format!("{}/{} ({})", 
            system_version.product_name, system_version.product_version, system_version.product_build_version);
        
        let result = output_path.join(os_symbol_folder);
        create_dir_all(&result).expect("Unable to create directory");
        result
    }

    fn calculate_temp_folder(os_symbol_path: &Path) -> PathBuf {
        let result = os_symbol_path.join("tmp");
        create_dir_all(&result).expect("Unable to create directory");
        result
    }
}

impl Drop for LibrarySymbolsArguments {
    fn drop(&mut self) {
        if remove_dir_all(&self.temp_path).is_err() {
            print!("Unable to delete temp folder");
        }
    }
}