mod dyld_shared_cache_extractor;
mod nm_library_symbols;
mod otool_library_symbols;
mod system_version;
mod who_am_i_user_name;

pub use dyld_shared_cache_extractor::*;
pub use nm_library_symbols::*;
pub use otool_library_symbols::*;
pub use system_version::*;
pub use who_am_i_user_name::*;

use std::{fs::{create_dir_all, File}, io::Write, path::Path, process::Output};

pub trait SaveOutput {
    fn get_raw_output(&self) -> &String;
}

fn parse_stdout(output: Output) -> Vec<String> {
    let raw_string = String::from_utf8(output.stdout).expect("Unable to save output");
    
    let mut vec: Vec<String> = Vec::new();
    for line in raw_string.lines() {
        vec.push(line.to_string());
    }

    vec
}

pub fn save_output_to_file<O: SaveOutput>(output: &O, file: &Path) {
    let raw_output = output.get_raw_output();
    if raw_output.is_empty() {
        return;
    }

    if let Some(parent_path) = file.parent() {
        create_dir_all(parent_path).expect("Unable to create directory");
    }

    let mut file = File::create(file).expect("Failed to create file to save output");
    file.write_all(raw_output.as_bytes()).expect("Failed to save cli output to file");
}