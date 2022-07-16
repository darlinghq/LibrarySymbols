use std::path::{Path,PathBuf};

use crate::argument::Arguments;
use crate::program::SystemVersionDefaults;

#[derive(Debug)]
pub struct Location {
    pub system_version_path: PathBuf,
    unique_version_path: Option<PathBuf>
}

impl Location {
    pub fn new(arguments: &Arguments) -> Location {
        let system_version_path: &Path = Path::new("/System/Library/CoreServices/SystemVersion");

        Location {
            system_version_path: arguments.path_from_base(system_version_path),
            unique_version_path: None
        }
    }

    pub fn initalize_unique_version_path(&mut self, arguments: &Arguments, system_version: &SystemVersionDefaults) {
        let version_folder = format!("{} ({})", system_version.product_version, system_version.product_build_version);
        let system_version = &system_version.product_name;

        self.unique_version_path = Some(arguments.path_from_results(Path::new(system_version.as_str())).join(version_folder))
    }
}