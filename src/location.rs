use std::fs::{read_dir, ReadDir};
use std::iter::Enumerate;
use std::path::{Path,PathBuf};

use crate::argument::Arguments;
use crate::program::SystemVersionDefaults;

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
pub struct BaseLocation {
    pub system_version_path: PathBuf,
    pub dyld_shardcache_macos_path: PathBuf,
    pub dyld_shardcache_iphoneos_path: PathBuf,
}

impl BaseLocation {
    pub fn new(arguments: &Arguments) -> BaseLocation {     
        let system_version_path = Path::new("System/Library/CoreServices/SystemVersion");
        let dyld_shardcache_macos =  Path::new("System/Library/dyld");
        let dyld_shardcache_iphoneos = Path::new("System/Library/Caches/com.apple.dyld");

        BaseLocation {
            system_version_path: arguments.path_from_base(system_version_path),
            dyld_shardcache_macos_path: arguments.path_from_base(dyld_shardcache_macos),
            dyld_shardcache_iphoneos_path: arguments.path_from_base(dyld_shardcache_iphoneos),
        }
    }
}

#[derive(Debug)]
pub struct ResultsLocation {
    pub shared_cache_path: PathBuf,
    pub unique_version_path: PathBuf,
    pub temp_shared_cache_path: PathBuf
}

impl ResultsLocation {
    pub fn new(arguments: &Arguments, system_version: &SystemVersionDefaults) -> ResultsLocation {
        const SHARED_CACHE_DIR: &str = "shared_cache";
        const TEMP_DIR: &str = "temp";

        let version_folder = format!("{} ({})", system_version.product_version, system_version.product_build_version);
        let system_version = &system_version.product_name;
        let unique_version_path = arguments.path_from_results(Path::new(system_version.as_str())).join(version_folder);
        let shared_cache_path = unique_version_path.join(SHARED_CACHE_DIR);

        let temp_shared_cache_path = unique_version_path.join(TEMP_DIR).join(SHARED_CACHE_DIR);

        ResultsLocation {
            shared_cache_path,
            unique_version_path,
            temp_shared_cache_path
        }
    }
}