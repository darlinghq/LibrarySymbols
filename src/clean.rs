use std::fs::remove_dir_all;

use crate::location::ResultsLocation;

pub fn remove_saved_symbols(results_locations: &ResultsLocation) {
    if let Ok(_) = remove_dir_all(&results_locations.os_version_path) {
        println!("Deleted {:?}", results_locations.os_version_path);
    }
}

pub fn remove_temp(results_locations: &ResultsLocation) {
    if let Ok(_) = remove_dir_all(&results_locations.temp_path) {
        println!("Cleaned up temp data");
    }
}