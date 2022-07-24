use std::fs::remove_dir_all;

use crate::location::ResultsLocation;

pub struct Cleanup {}

impl Cleanup {
    pub fn preclean(results_locations: &ResultsLocation) {
        if let Ok(_) = remove_dir_all(&results_locations.unique_version_path) {
            println!("Deleted {:?}", results_locations.unique_version_path);
        }
    }

    pub fn postclean(results_locations: &ResultsLocation) {
        if let Ok(_) = remove_dir_all(&results_locations.temp_path) {
            println!("Cleaned up temp data");
        }
    }
}